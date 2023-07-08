use crate::rules::Rules;
use toml::Value;
use regex::Regex;

fn maybe_remove_parentheses(rules: &Rules, txt: &str) -> String {
    let mut replaced = txt.to_string();
    if rules.remove_parentheses {
        let regex = Regex::new("\\([^\\()\n]*\\)").unwrap();
        replaced = regex.replace_all(txt, "").to_string().replace("  ", " ");
    }
    return replaced;
}

pub fn replace_strings(rules: &Rules, raw: &str) -> String {
    let mut result = maybe_remove_parentheses(&rules, raw.trim());

    for replacement_rules in rules.replacements.iter() {
        if Value::as_array(replacement_rules).unwrap().len() == 2 {
            let abbreviation = replacement_rules[0].as_str().unwrap();
            let replacement = replacement_rules[1].as_str().unwrap();
            result = result.replace(abbreviation, replacement);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use toml::Value;
    use toml::value::Array;

    #[test]
    fn test_nothing() {
        let rules : Rules = Rules {
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me&You");
    }

    #[test]
    fn test_nothing_if_replacement_missing() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me&You");
    }

    #[test]
    fn test_nothing_if_empty_rules() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from::<Array>(vec![]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me&You");
    }

    #[test]
    fn test_one_abbreviation() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from("and").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "MeandYou");
    }

    #[test]
    fn test_one_abbreviation_whitespace() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from(" & ").unwrap(), Value::try_from(" and ").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me & You")), "Me and You");
    }

    #[test]
    fn test_one_abbreviation_mixed() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from(" and ").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me and You");
    }

    #[test]
    fn test_multiple_occurances() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from("and").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me & You & Them")), "Me and You and Them");
    }

    #[test]
    fn test_multiple_abbreviations() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from(" and ").unwrap()]).unwrap(),
                Value::try_from([Value::try_from("etc.").unwrap(), Value::try_from("et cetera").unwrap()]).unwrap(),
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me and You");
        assert_eq!(replace_strings(&rules, &String::from("Me&You etc.")), "Me and You et cetera");
    }

    #[test]
    fn test_replace_empty() {
        let rules : Rules = Rules {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from("").unwrap()]).unwrap(),
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "MeYou");
    }

    #[test]
    fn test_remove_parentheses() {
        let mut rules : Rules = Rules {
            remove_parentheses: false,
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("First (content) should stay.")), "First (content) should stay.");
        assert_ne!(replace_strings(&rules, &String::from("Second (content) should stay.")), "Second should stay.");

        rules = Rules {
            remove_parentheses: true,
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Third (content) should be removed.")), "Third should be removed.");
        assert_eq!(replace_strings(&rules, &String::from("Fourth (content (and nested one)) only nested should be removed.")), "Fourth (content ) only nested should be removed.");
        assert_eq!(replace_strings(&rules, &String::from("Fifth [content (and nested one)] should partly be removed.")), "Fifth [content ] should partly be removed.");
        assert_ne!(replace_strings(&rules, &String::from("Sixth (content \n on \n multiple lines) should not be removed.")), "Sixth should not be removed.");
    }
}
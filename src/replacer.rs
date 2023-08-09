use crate::rules::Rules;
use toml::Value;
use regex::Regex;

pub fn replace_strings(rules: &Rules, raw: &str) -> String {
    let mut result = raw.trim().to_string();

    // bracket removal
    for bracket_pair in rules.remove_brackets_list.iter() {
        if Value::as_array(bracket_pair).unwrap().len() == 2 {
            let br0 = bracket_pair[0].as_str().unwrap();
            let br1 = bracket_pair[1].as_str().unwrap();
            let regex = Regex::new(format!(r#"\{}[^\{}\{}]*\{}"#, br0, br0, br1, br1).as_str()).unwrap();
            let mut prev = String::from("");
            while prev != result {
                prev = result.clone();
                result = regex.replace_all(prev.as_str(), " ").to_string().replace("  ", " ");
            }
        }
    }

    // replacements
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
    fn test_remove_brackets_list_empty() {
        let rules : Rules = Rules {
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("One: (content) should stay.")), "One: (content) should stay.");
    }

    #[test]
    fn test_remove_brackets_list() {
        let rules = Rules {
            remove_brackets_list: vec![
                Value::try_from([Value::try_from("(").unwrap(), Value::try_from(")").unwrap()]).unwrap(),
                Value::try_from([Value::try_from("[").unwrap(), Value::try_from("]").unwrap()]).unwrap(),
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Two: (content) should be removed.")), "Two: should be removed.");
        assert_eq!(replace_strings(&rules, &String::from("Three: [content (and nested one)] should be removed.")), "Three: should be removed.");
        assert_eq!(replace_strings(&rules, &String::from("Four: (content (and nested one)) should be removed.")), "Four: should be removed.");
        assert_eq!(replace_strings(&rules, &String::from("Five: (one) (two) and [three] 'and' should stay.")), "Five: and 'and' should stay.");
    }
}
use crate::config::Config;
use toml::Value;

pub fn replace_strings(rules: &Config, raw: &str) -> String {
    let mut result = raw.trim().to_string();

    for replacement_config in rules.replacements.iter() {
        if Value::as_array(replacement_config).unwrap().len() == 2 {
            let abbreviation = replacement_config[0].as_str().unwrap();
            let replacement = replacement_config[1].as_str().unwrap();
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
        let rules : Config = Config {
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me&You");
    }

    #[test]
    fn test_nothing_if_replacement_missing() {
        let rules : Config = Config {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me&You");
    }

    #[test]
    fn test_nothing_if_empty_config() {
        let rules : Config = Config {
            replacements: vec![
                Value::try_from::<Array>(vec![]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me&You");
    }

    #[test]
    fn test_one_abbreviation() {
        let rules : Config = Config {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from("and").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "MeandYou");
    }

    #[test]
    fn test_one_abbreviation_whitespace() {
        let rules : Config = Config {
            replacements: vec![
                Value::try_from([Value::try_from(" & ").unwrap(), Value::try_from(" and ").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me & You")), "Me and You");
    }

    #[test]
    fn test_one_abbreviation_mixed() {
        let rules : Config = Config {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from(" and ").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "Me and You");
    }

    #[test]
    fn test_multiple_occurances() {
        let rules : Config = Config {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from("and").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me & You & Them")), "Me and You and Them");
    }

    #[test]
    fn test_multiple_abbreviations() {
        let rules : Config = Config {
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
        let rules : Config = Config {
            replacements: vec![
                Value::try_from([Value::try_from("&").unwrap(), Value::try_from("").unwrap()]).unwrap(),
            ],
            ..Default::default()
        };

        assert_eq!(replace_strings(&rules, &String::from("Me&You")), "MeYou");
    }
}
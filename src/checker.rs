use crate::config::Config;
use toml::Value;
use itertools::join;
use regex::Regex;

pub fn check(rules: &Config, raw: &&str) -> bool {
    let trimmed = raw.trim();
    if trimmed.len() < rules.min_trimmed_length
        || rules.quote_start_with_alphanumeric
            && trimmed.chars().nth(0) == Some('"')
            && trimmed
                .chars()
                .nth(1)
                .map(|c| !c.is_alphabetic())
                .unwrap_or_default()
        || trimmed.chars().filter(|c| c.is_alphabetic()).count() < rules.min_alphanumeric_characters
        || rules.needs_punctuation_end && trimmed.ends_with(|c: char| c.is_alphabetic())
        || rules.needs_alphanumeric_start && trimmed.starts_with(|c: char| !c.is_alphabetic())
        || rules.needs_uppercase_start && trimmed.starts_with(|c: char| c.is_lowercase())
    {
        return false;
    }
    let symbols = trimmed.chars().any(|c| {
        rules.disallowed_symbols.contains(&Value::try_from(c).unwrap())
    });
    if symbols {
        return false;
    }
    if rules.broken_whitespace.iter().any(|broken| raw.contains(Value::as_str(broken).unwrap())) {
        return false;
    }
    let words = trimmed.split_whitespace();
    let word_count = words.clone().count();
    let s = join(words, " ");
    if word_count < rules.min_word_count || word_count > rules.max_word_count {
        return false;
    }
    let abrv = Regex::new(r"[[:upper:]]+\.*[[:upper:]]")
        .unwrap()
        .is_match(&s);
    if abrv {
        return false;
    }
    let numbers = s.contains(char::is_numeric);
    if numbers {
        return false;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::load_config;
    use toml::Value;

    #[test]
    fn test_min_trimmed_length() {
        let rules : Config = Config {
            min_trimmed_length: 3,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"  aa     "), false);
        assert_eq!(check(&rules, &"  aaa     "), true);
    }

    #[test]
    fn test_min_word_count() {
        let rules : Config = Config {
            min_word_count: 2,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"one"), false);
        assert_eq!(check(&rules, &"two words"), true);
    }

    #[test]
    fn test_max_word_count() {
        let rules : Config = Config {
            max_word_count: 2,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"three words now"), false);
        assert_eq!(check(&rules, &"two words"), true);
    }

    #[test]
    fn test_min_alphanumeric_characters() {
        let rules : Config = Config {
            min_alphanumeric_characters: 3,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"no!!"), false);
        assert_eq!(check(&rules, &"yes!"), true);
    }

    #[test]
    fn test_may_end_with_colon() {
        let mut rules : Config = Config {
            may_end_with_colon: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"ends with colon:"), false);

        rules = Config {
            may_end_with_colon: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"ends with colon:"), true);
    }

    #[test]
    fn test_quote_start_with_alphanumeric() {
        let mut rules : Config = Config {
            quote_start_with_alphanumeric: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"\"😊"), true);

        rules = Config {
            quote_start_with_alphanumeric: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"\"😊"), false);
    }

    #[test]
    fn test_needs_punctuation_end() {
        let mut rules : Config = Config {
            needs_punctuation_end: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"This has no punctuation"), true);
        assert_eq!(check(&rules, &"This has punctuation."), true);

        rules = Config {
            needs_punctuation_end: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"This has no punctuation"), false);
        assert_eq!(check(&rules, &"This has punctuation."), true);
    }

    #[test]
    fn test_needs_alphanumeric_start() {
        let mut rules : Config = Config {
            needs_alphanumeric_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"?Foo"), true);
        assert_eq!(check(&rules, &"This has a normal start"), true);

        rules = Config {
            needs_alphanumeric_start: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"?Foo"), false);
        assert_eq!(check(&rules, &"This has a normal start"), true);
    }

    #[test]
    fn test_needs_uppercase_start() {
        let mut rules : Config = Config {
            needs_uppercase_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"foo"), true);
        assert_eq!(check(&rules, &"Foo"), true);

        rules = Config {
            needs_uppercase_start: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"foo"), false);
        assert_eq!(check(&rules, &"Foo"), true);
    }

    #[test]
    fn test_disallowed_symbols() {
        let rules : Config = Config {
            disallowed_symbols: vec![Value::try_from('%').unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &"This has no percentage but other & characters"), true);
        assert_eq!(check(&rules, &"This has a %"), false);
    }

    #[test]
    fn test_broken_whitespace() {
        let rules : Config = Config {
            broken_whitespace: vec![Value::try_from("  ").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &"This has no broken whitespace"), true);
        assert_eq!(check(&rules, &"This has  broken whitespace"), false);
    }

    #[test]
    fn test_abbreviation_patterns() {
        let rules : Config = Config {
            abbreviation_patterns: vec![Value::try_from("[A-Z]{2}").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &"This no two following uppercase letters"), true);
        assert_eq!(check(&rules, &"This has two FOllowing uppercase letters"), false);
    }

    #[test]
    fn test_english() {
        let rules : Config = load_config("english");

        assert_eq!(check(&rules, &""), false);
        assert_eq!(check(&rules, &"\"Some test"), true);
        assert_eq!(check(&rules, &"\"😊"), false);
        assert_eq!(check(&rules, &"This ends with:"), false);
        assert_eq!(check(&rules, &" AA "), false);
        assert_eq!(check(&rules, &"This has broken  space"), false);
        assert_eq!(check(&rules, &"This as well !"), false);
        assert_eq!(check(&rules, &"And this ;"), false);
        assert_eq!(check(&rules, &"This is gonna be way way way way way way way way way way too long"), false);
        assert_eq!(check(&rules, &"This is absolutely valid."), true);
        assert_eq!(check(&rules, &"This contains 1 number"), false);
        assert_eq!(check(&rules, &"foo\n\nfoo"), false);
        assert_eq!(check(&rules, &"foo<>"), false);
        assert_eq!(check(&rules, &"foo*@"), false);
        assert_eq!(check(&rules, &"A.B"), false);
        assert_eq!(check(&rules, &r#""S.T.A.L.K.E.R."#), false);
    }

    #[test]
    fn test_french() {
        let rules : Config = load_config("french");

        assert_eq!(check(&rules, &""), false);
        assert_eq!(check(&rules, &"\"😊"), false);
        assert_eq!(check(&rules, &"This ends with:"), false);
        assert_eq!(check(&rules, &"This does not end with a period"), false);
        assert_eq!(check(&rules, &"?This does not start with a alphanumeric letter"), false);
        assert_eq!(check(&rules, &"this starts with lowercase"), false);
        assert_eq!(check(&rules, &" AA "), false);
        assert_eq!(check(&rules, &"This has broken  space"), false);
        assert_eq!(check(&rules, &"This as well !"), false);
        assert_eq!(check(&rules, &"And this ;"), false);
        assert_eq!(check(&rules, &"This is gonna be way way way way way way way way way way too long"), false);
        assert_eq!(check(&rules, &"Short"), false);
        assert_eq!(check(&rules, &"This is absolutely validé."), true);
        assert_eq!(check(&rules, &"No!!!"), false);
        assert_eq!(check(&rules, &"This contains 1 number"), false);
        assert_eq!(check(&rules, &"foo\n\nfoo"), false);
        assert_eq!(check(&rules, &"foo<>"), false);
        assert_eq!(check(&rules, &"foo«"), false);
        assert_eq!(check(&rules, &"foo*@"), false);
        assert_eq!(check(&rules, &"A.B"), false);
        assert_eq!(check(&rules, &"A."), false);
        assert_eq!(check(&rules, &"Some sentence that ends with A."), false);
        assert_eq!(check(&rules, &r#""S.T.A.L.K.E.R."#), false);
    }
}

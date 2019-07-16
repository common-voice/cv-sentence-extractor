use crate::config::Config;
use toml::Value;
use regex::Regex;

pub fn check(rules: &Config, raw: &&str) -> bool {
    let trimmed = raw.trim();
    if trimmed.len() < rules.min_trimmed_length
        || rules.quote_start_with_letter
            && trimmed.chars().nth(0) == Some('"')
            && trimmed
                .chars()
                .nth(1)
                .map(|c| !c.is_alphabetic())
                .unwrap_or_default()
        || trimmed.chars().filter(|c| c.is_alphabetic()).count() < rules.min_characters
        || !rules.may_end_with_colon && trimmed.ends_with(':')
        || rules.needs_punctuation_end && trimmed.ends_with(|c: char| c.is_alphabetic())
        || rules.needs_letter_start && trimmed.starts_with(|c: char| !c.is_alphabetic())
        || rules.needs_uppercase_start && trimmed.starts_with(|c: char| c.is_lowercase())
        || trimmed.contains("\n")
        || trimmed.contains(char::is_numeric)
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
    if word_count < rules.min_word_count
        || word_count > rules.max_word_count
        || words.into_iter().any(|word| rules.disallowed_words.contains(&Value::from(word)))
    {
        return false;
    }

    let abbr = rules.abbreviation_patterns.iter().any(|pattern| {
        let regex = Regex::new(Value::as_str(pattern).unwrap()).unwrap();
        regex.is_match(&trimmed)
    });
    if abbr {
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
    fn test_min_characters() {
        let rules : Config = Config {
            min_characters: 3,
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
    fn test_quote_start_with_letter() {
        let mut rules : Config = Config {
            quote_start_with_letter: false,
            needs_letter_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"\"😊 foo"), true);

        rules = Config {
            quote_start_with_letter: true,
            needs_letter_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"\"😊 foo"), false);
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
    fn test_needs_letter_start() {
        let mut rules : Config = Config {
            needs_letter_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &"?Foo"), true);
        assert_eq!(check(&rules, &"This has a normal start"), true);

        rules = Config {
            needs_letter_start: true,
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
    fn test_disallowed_words() {
        let rules : Config = Config {
            disallowed_words: vec![Value::try_from("blerg").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &"This has blerg"), false);
        assert_eq!(check(&rules, &"This hasn't bl e r g"), true);
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
        assert_eq!(check(&rules, &"\"😊"), false);
        assert_eq!(check(&rules, &"This ends with:"), false);
        assert_eq!(check(&rules, &" AA "), false);
        assert_eq!(check(&rules, &"This has broken  space"), false);
        assert_eq!(check(&rules, &"This as well !"), false);
        assert_eq!(check(&rules, &"And this ;"), false);
        assert_eq!(check(&rules, &"This is gonna be way way way way way way way way way way too long"), false);
        assert_eq!(check(&rules, &"This is absolutely valid."), true);
        assert_eq!(check(&rules, &"This contains 1 number"), false);
        assert_eq!(check(&rules, &"this is lowercase"), true);
        assert_eq!(check(&rules, &"foo\n\nfoo"), false);
        assert_eq!(check(&rules, &"foo\\foo"), false);
        assert_eq!(check(&rules, &"foo<>"), false);
        assert_eq!(check(&rules, &"foo*@"), false);
        assert_eq!(check(&rules, &"A.B"), false);
        assert_eq!(check(&rules, &"S.T.A.L.K.E.R."), false);
    }

    #[test]
    fn test_french() {
        let rules : Config = load_config("french");

        assert_eq!(check(&rules, &""), false);
        assert_eq!(check(&rules, &"\"😊"), false);
        assert_eq!(check(&rules, &"This ends with:"), false);
        assert_eq!(check(&rules, &"This does not end with a period"), false);
        assert_eq!(check(&rules, &"?This does not start with a letter"), false);
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
        assert_eq!(check(&rules, &"S.T.A.L.K.E.R."), false);
        assert_eq!(check(&rules, &"Some sentence that ends with A."), false);
    }

    #[test]
    fn test_german() {
        let rules : Config = load_config("german");

        assert_eq!(check(&rules, &"Dies ist ein korrekter Satz."), true);
        assert_eq!(check(&rules, &"Satzzeichen in der Mitte. Wird nicht akzeptiert."), false);
        assert_eq!(check(&rules, &"Satzzeichen in der Mitte? Wird nicht akzeptiert."), false);
        assert_eq!(check(&rules, &"Satzzeichen in der Mitte! Wird nicht akzeptiert."), false);
        assert_eq!(check(&rules, &"Französische Satzzeichen werden ignorierté."), false);
        assert_eq!(check(&rules, &"Andere Satzzeichen wie Åblabla werden auch ignoriert."), false);
        assert_eq!(check(&rules, &"Γεια σας"), false);
        assert_eq!(check(&rules, &"Sätze dürfen keine Wörter mit nur einem B Buchstaben haben."), false);
        assert_eq!(check(&rules, &"A auch nicht am Anfang."), false);
        assert_eq!(check(&rules, &"Oder am Ende e."), false);
        assert_eq!(check(&rules, &"Oder am Ende e."), false);
        assert_eq!(check(&rules, &"AmSi ist eine schwarze Masse, isomorph mit LaSi"), false);
        assert_eq!(check(&rules, &"Die Aussperrung ist nach Art."), false);
        assert_eq!(check(&rules, &"Remy & Co."), false);
        assert_eq!(check(&rules, &"Es ist die sog."), false);
    }
}

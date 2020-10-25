use crate::rules::Rules;
use toml::Value;
use regex::Regex;

pub fn check(rules: &Rules, raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.len() < rules.min_trimmed_length
        || rules.quote_start_with_letter
            && trimmed.starts_with('"')
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
        || trimmed.contains('\n')
        || trimmed.contains(char::is_numeric)
    {
        return false;
    }

    let invalid_symbols = if !rules.allowed_symbols_regex.is_empty() {
            let regex = Regex::new(&rules.allowed_symbols_regex).unwrap();
            trimmed.chars().any(|c| {
                !regex.is_match(&c.to_string())
            })
        } else {
            trimmed.chars().any(|c| {
                rules.disallowed_symbols.contains(&Value::try_from(c).unwrap())
            })
        };

    if invalid_symbols {
        return false;
    }

    if rules.broken_whitespace.iter().any(|broken| trimmed.contains(Value::as_str(broken).unwrap())) {
        return false;
    }

    let mut words = trimmed.split_whitespace();
    let word_count = words.clone().count();
    if word_count < rules.min_word_count
        || word_count > rules.max_word_count
        || words.any(|word| rules.disallowed_words.contains(
             &word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase()
           ))
    {
        return false;
    }

    let abbr = rules.abbreviation_patterns.iter().any(|pattern| {
        let regex = Regex::new(Value::as_str(pattern).unwrap()).unwrap();
        regex.is_match(&trimmed)
    });
    let other = rules.other_patterns.iter().any(|pattern| {
        let regex = Regex::new(Value::as_str(pattern).unwrap()).unwrap();
        regex.is_match(&trimmed)
    });
    if abbr || other {
        return false;
    }

    if !rules.even_symbols.is_empty() {
        let has_uneven_symbols = rules.even_symbols.iter().any(|even_symbol| {
            let count = trimmed.matches(Value::as_str(even_symbol).unwrap()).count();
            count % 2 != 0
        });
        if has_uneven_symbols {
            return false;
        }
    }

    if !rules.matching_symbols.is_empty() {
        let has_unmatching_symbols = rules.matching_symbols.iter().any(|match_symbol| {
            let first_count = trimmed.matches(match_symbol[0].as_str().unwrap()).count();
            let second_count = trimmed.matches(match_symbol[1].as_str().unwrap()).count();
            first_count != second_count
        });
        if has_unmatching_symbols {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rules::load_rules;
    use toml::Value;

    #[test]
    fn test_min_trimmed_length() {
        let rules : Rules = Rules {
            min_trimmed_length: 3,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("  aa     ")), false);
        assert_eq!(check(&rules, &String::from("  aaa     ")), true);
    }

    #[test]
    fn test_min_word_count() {
        let rules : Rules = Rules {
            min_word_count: 2,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("one")), false);
        assert_eq!(check(&rules, &String::from("two words")), true);
    }

    #[test]
    fn test_max_word_count() {
        let rules : Rules = Rules {
            max_word_count: 2,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("three words now")), false);
        assert_eq!(check(&rules, &String::from("two words")), true);
    }

    #[test]
    fn test_min_characters() {
        let rules : Rules = Rules {
            min_characters: 3,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("no!!")), false);
        assert_eq!(check(&rules, &String::from("yes!")), true);
    }

    #[test]
    fn test_may_end_with_colon() {
        let mut rules : Rules = Rules {
            may_end_with_colon: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("ends with colon:")), false);

        rules = Rules {
            may_end_with_colon: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("ends with colon:")), true);
    }

    #[test]
    fn test_quote_start_with_letter() {
        let mut rules : Rules = Rules {
            quote_start_with_letter: false,
            needs_letter_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("\"😊 foo")), true);

        rules = Rules {
            quote_start_with_letter: true,
            needs_letter_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("\"😊 foo")), false);
    }

    #[test]
    fn test_needs_punctuation_end() {
        let mut rules : Rules = Rules {
            needs_punctuation_end: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has no punctuation")), true);
        assert_eq!(check(&rules, &String::from("This has punctuation.")), true);

        rules = Rules {
            needs_punctuation_end: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has no punctuation")), false);
        assert_eq!(check(&rules, &String::from("This has punctuation.")), true);
    }

    #[test]
    fn test_needs_letter_start() {
        let mut rules : Rules = Rules {
            needs_letter_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("?Foo")), true);
        assert_eq!(check(&rules, &String::from("This has a normal start")), true);

        rules = Rules {
            needs_letter_start: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("?Foo")), false);
        assert_eq!(check(&rules, &String::from("This has a normal start")), true);
    }

    #[test]
    fn test_needs_uppercase_start() {
        let mut rules : Rules = Rules {
            needs_uppercase_start: false,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("foo")), true);
        assert_eq!(check(&rules, &String::from("Foo")), true);

        rules = Rules {
            needs_uppercase_start: true,
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("foo")), false);
        assert_eq!(check(&rules, &String::from("Foo")), true);
    }

    #[test]
    fn test_disallowed_symbols() {
        let rules : Rules = Rules {
            disallowed_symbols: vec![Value::try_from('%').unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has no percentage but other & characters")), true);
        assert_eq!(check(&rules, &String::from("This has a %")), false);
    }

    #[test]
    fn test_allowed_symbols_regex() {
        let rules : Rules = Rules {
            allowed_symbols_regex: String::from("[\u{0020}-\u{005A}]"),
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("ONLY UPPERCASE AND SPACE IS ALLOWED")), true);
        assert_eq!(check(&rules, &String::from("This is not uppercase")), false);
    }

    #[test]
    fn test_allowed_symbols_regex_over_disallowed() {
        let rules : Rules = Rules {
            allowed_symbols_regex: String::from("[\u{0020}-\u{005A}]"),
            disallowed_symbols: vec![Value::try_from('O').unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("ONLY UPPERCASE AND SPACE IS ALLOWED AND DISALLOWED O IS OKAY")), true);
    }

    #[test]
    fn test_disallowed_words() {
        let rules : Rules = Rules {
            disallowed_words: ["blerg"].iter().map(|s| (*s).to_string()).collect(),
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has blerg")), false);
        assert_eq!(check(&rules, &String::from("This has a capital bLeRg")), false);
        assert_eq!(check(&rules, &String::from("This has many blergs blerg blerg blerg")), false);
        assert_eq!(check(&rules, &String::from("Here is a blerg, with comma")), false);
        assert_eq!(check(&rules, &String::from("This hasn't bl e r g")), true);

        let rules : Rules = Rules {
            disallowed_words: ["a's"].iter().map(|s| (*s).to_string()).collect(),
            ..Default::default()
        };
        assert_eq!(check(&rules, &String::from("This has a's")), false);
    }

    #[test]
    fn test_broken_whitespace() {
        let rules : Rules = Rules {
            broken_whitespace: vec![Value::try_from("  ").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has no broken whitespace")), true);
        assert_eq!(check(&rules, &String::from("This has  broken whitespace")), false);
    }

    #[test]
    fn test_abbreviation_patterns() {
        let rules : Rules = Rules {
            abbreviation_patterns: vec![Value::try_from("[A-Z]{2}").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This no two following uppercase letters")), true);
        assert_eq!(check(&rules, &String::from("This has two FOllowing uppercase letters")), false);
    }

    #[test]
    fn test_other_patterns_long_words() {
        let rules : Rules = Rules {
            other_patterns: vec![Value::try_from("\\w{5,50}").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("ये कलाकृतियां खजुराहो मंदिर की कलाकृतियों की याद दिलाती हैं.")), false);
        assert_eq!(check(&rules, &String::from("φφδφξασκ")), false);
        assert_eq!(check(&rules, &String::from("No long test")), true);
        assert_eq!(check(&rules, &String::from("Longlong test this is")), false);
        assert_eq!(check(&rules, &String::from("This is longlong test")), false);
        assert_eq!(check(&rules, &String::from("This is test which is longlong")), false);
    }

    #[test]
    fn test_uneven_quotes_allowed_default() {
        let rules : Rules = Rules {
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has \"uneven quotes and it is fine!")), true);
    }

    #[test]
    fn test_uneven_quotes_allowed() {
        let rules : Rules = Rules {
            even_symbols: vec![],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has \"uneven quotes and it is fine!")), true);
        assert_eq!(check(&rules, &String::from("This has (uneven parenthesis and it is fine!")), true);
    }

    #[test]
    fn test_uneven_quotes_not_allowed() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap(), Value::try_from("(").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has \"uneven quotes and it is not fine!")), false);
        assert_eq!(check(&rules, &String::from("This has (uneven parenthesis and it is not fine!")), false);
    }

    #[test]
    fn test_uneven_quotes_not_allowed_even() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has \"even\" quotes and it is fine!")), true);
    }

    #[test]
    fn test_uneven_quotes_not_allowed_multiple() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap(), Value::try_from("'").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has \"uneven quotes' and it is fine!")), false);
    }

    #[test]
    fn test_uneven_quotes_not_allowed_multiple_one_ok() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap(), Value::try_from("'").unwrap()],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This has \"uneven\" quotes' and it is fine!")), false);
    }

    #[test]
    fn test_matching_quotes_valid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This is „a quote“")), true);
    }

    #[test]
    fn test_matching_quotes_invalid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This is „a quote")), false);
    }

    #[test]
    fn test_matching_quotes_valid_multiple() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This is „a quote“ and „another one“")), true);
    }

    #[test]
    fn test_matching_quotes_invalid_multiple() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This is „a quote“ and another one“")), false);
    }

    #[test]
    fn test_matching_bracket_valid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("(").unwrap(), Value::try_from("]").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This is (a bracket]")), true);
    }

    #[test]
    fn test_matching_bracket_invalid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("(").unwrap(), Value::try_from("]").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert_eq!(check(&rules, &String::from("This is (a bracket")), false);
    }

    #[test]
    fn test_english() {
        let rules : Rules = load_rules("en");

        assert_eq!(check(&rules, &String::from("")), false);
        assert_eq!(check(&rules, &String::from("\"😊")), false);
        assert_eq!(check(&rules, &String::from("This ends with:")), false);
        assert_eq!(check(&rules, &String::from(" AA ")), false);
        assert_eq!(check(&rules, &String::from("This has broken  space")), false);
        assert_eq!(check(&rules, &String::from("This as well !")), false);
        assert_eq!(check(&rules, &String::from("And this ;")), false);
        assert_eq!(check(&rules, &String::from("This is gonna be way way way way way way way way way way too long")), false);
        assert_eq!(check(&rules, &String::from("This is absolutely valid.")), true);
        assert_eq!(check(&rules, &String::from("This contains 1 number")), false);
        assert_eq!(check(&rules, &String::from("this is lowercase")), true);
        assert_eq!(check(&rules, &String::from("foo\n\nfoo")), false);
        assert_eq!(check(&rules, &String::from("foo\\foo")), false);
        assert_eq!(check(&rules, &String::from("foo<>")), false);
        assert_eq!(check(&rules, &String::from("foo*@")), false);
        assert_eq!(check(&rules, &String::from("A.B")), false);
        assert_eq!(check(&rules, &String::from("S.T.A.L.K.E.R.")), false);
    }

    #[test]
    fn test_french() {
        let rules : Rules = load_rules("fr");

        assert_eq!(check(&rules, &String::from("")), false);
        assert_eq!(check(&rules, &String::from("\"😊")), false);
        assert_eq!(check(&rules, &String::from("This ends with:")), false);
        assert_eq!(check(&rules, &String::from("This does not end with a period")), false);
        assert_eq!(check(&rules, &String::from("?This does not start with a letter")), false);
        assert_eq!(check(&rules, &String::from("this starts with lowercase")), false);
        assert_eq!(check(&rules, &String::from(" AA ")), false);
        assert_eq!(check(&rules, &String::from("This has broken  space")), false);
        assert_eq!(check(&rules, &String::from("This as well !")), false);
        assert_eq!(check(&rules, &String::from("And this ;")), false);
        assert_eq!(check(&rules, &String::from("This is gonna be way way way way way way way way way way too long")), false);
        assert_eq!(check(&rules, &String::from("Short")), false);
        assert_eq!(check(&rules, &String::from("This is absolutely validé.")), true);
        assert_eq!(check(&rules, &String::from("No!!!")), false);
        assert_eq!(check(&rules, &String::from("This contains 1 number")), false);
        assert_eq!(check(&rules, &String::from("foo\n\nfoo")), false);
        assert_eq!(check(&rules, &String::from("foo<>")), false);
        assert_eq!(check(&rules, &String::from("foo«")), false);
        assert_eq!(check(&rules, &String::from("foo*@")), false);
        assert_eq!(check(&rules, &String::from("A.B")), false);
        assert_eq!(check(&rules, &String::from("S.T.A.L.K.E.R.")), false);
        assert_eq!(check(&rules, &String::from("Some sentence that ends with A.")), false);
    }

    #[test]
    fn test_german() {
        let rules : Rules = load_rules("de");

        assert_eq!(check(&rules, &String::from("Dies ist ein korrekter Satz.")), true);
        assert_eq!(check(&rules, &String::from("Satzzeichen in der Mitte. Wird nicht akzeptiert.")), false);
        assert_eq!(check(&rules, &String::from("Satzzeichen in der Mitte? Wird nicht akzeptiert.")), false);
        assert_eq!(check(&rules, &String::from("Satzzeichen in der Mitte! Wird nicht akzeptiert.")), false);
        assert_eq!(check(&rules, &String::from("Französische Satzzeichen werden ignorierté.")), false);
        assert_eq!(check(&rules, &String::from("Andere Satzzeichen wie Åblabla werden auch ignoriert.")), false);
        assert_eq!(check(&rules, &String::from("Γεια σας")), false);
        assert_eq!(check(&rules, &String::from("Sätze dürfen keine Wörter mit nur einem B Buchstaben haben.")), false);
        assert_eq!(check(&rules, &String::from("A auch nicht am Anfang.")), false);
        assert_eq!(check(&rules, &String::from("Oder am Ende e.")), false);
        assert_eq!(check(&rules, &String::from("Oder am Ende e.")), false);
        assert_eq!(check(&rules, &String::from("AmSi ist eine schwarze Masse, isomorph mit LaSi")), false);
        assert_eq!(check(&rules, &String::from("Die Aussperrung ist nach Art.")), false);
        assert_eq!(check(&rules, &String::from("Remy & Co.")), false);
        assert_eq!(check(&rules, &String::from("Es ist die sog.")), false);
        assert_eq!(check(&rules, &String::from("Kein deutsches Wort: ambiguous.")), false);
        assert_eq!(check(&rules, &String::from("Bundesliga am Anfang eines Satzes.")), false);
        assert_eq!(check(&rules, &String::from("Liga am Anfang eines Satzes.")), false);
        assert_eq!(check(&rules, &String::from("Abkürzung am Ende hl.")), false);
        assert_eq!(check(&rules, &String::from("Abkürzung am Ende geb.")), false);
    }

    #[test]
    fn test_hungarian() {
        let rules : Rules = load_rules("hu");

        assert_eq!(check(&rules, &String::from("A BBC Rádió rádiójátékot készített belőle.")), false);
        assert_eq!(check(&rules, &String::from("A BD fejlesztései miatt verziószámmal is találkozhatunk.")), false);
        assert_eq!(check(&rules, &String::from("A BCS-elmélet más fermionok közti kölcsönhatások leírására is alkalmas.")), false);
        assert_eq!(check(&rules, &String::from("A BKV-nál a kocsik elbontásáról döntöttek.")), false);
        assert_eq!(check(&rules, &String::from("A BL-ben ötször játszhatott.")), false);
        assert_eq!(check(&rules, &String::from("A B-döntőt hat résztvevővel rendezték.")), false);
        assert_eq!(check(&rules, &String::from("A -ház egyik legkiválóbb uralkodójaként tartják számon.")), false);
        assert_eq!(check(&rules, &String::from("A egyik legkiválóbb uralkodójaként tartják számon.")), true);
    }
}

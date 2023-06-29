use crate::rules::Rules;
use toml::Value;
use regex::Regex;

fn in_limit(x: usize, min_val: usize, max_val: usize) -> bool {
    x >= min_val && x <= max_val
}

pub fn check(rules: &Rules, raw: &str) -> bool {
    let trimmed = raw.trim();
    let alpha_cnt = trimmed.chars().filter(|c| c.is_alphabetic()).count();
    if trimmed.len() < rules.min_trimmed_length
        || rules.quote_start_with_letter
            && trimmed.starts_with('"')
            && trimmed
                .chars()
                .nth(1)
                .map(|c| !c.is_alphabetic())
                .unwrap_or_default()
        || !in_limit(alpha_cnt, rules.min_characters, rules.max_characters)
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

    let words = trimmed.split_whitespace();
    let word_count = words.clone().count();
    if word_count < rules.min_word_count
        || word_count > rules.max_word_count
        || words.clone().any(|word| rules.disallowed_words.contains(
             &word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase()
           ))
    {
        return false;
    }

    if !rules.stem_separator_regex.is_empty() {
        let regex: Regex = Regex::new(&rules.stem_separator_regex).unwrap();
        let mut stems_words: Vec<&str> = vec![];
        
        for word in words {
            let maybe_stem_word = regex.split(word).next().unwrap_or(word);
            if maybe_stem_word != word {
                stems_words.push(maybe_stem_word);
            }
        }

        if stems_words.into_iter().any(|word| rules.disallowed_words.contains(
            &word.to_lowercase()
        )) {
            return false;
        }
    }

    let abbr = rules.abbreviation_patterns.iter().any(|pattern| {
        let regex = Regex::new(Value::as_str(pattern).unwrap()).unwrap();
        regex.is_match(trimmed)
    });
    let other = rules.other_patterns.iter().any(|pattern| {
        let regex = Regex::new(Value::as_str(pattern).unwrap()).unwrap();
        regex.is_match(trimmed)
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

        assert!(!check(&rules, &String::from("  aa     ")));
        assert!(check(&rules, &String::from("  aaa     ")));
    }

    #[test]
    fn test_min_word_count() {
        let rules : Rules = Rules {
            min_word_count: 2,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("one")));
        assert!(check(&rules, &String::from("two words")));
    }

    #[test]
    fn test_max_word_count() {
        let rules : Rules = Rules {
            max_word_count: 2,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("three words now")));
        assert!(check(&rules, &String::from("two words")));
    }

    #[test]
    fn test_min_characters() {
        let rules : Rules = Rules {
            min_characters: 3,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("no!!")));
        assert!(check(&rules, &String::from("yes!")));
    }

    #[test]
    fn test_max_characters() {
        let rules : Rules = Rules {
            max_characters: 25,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This is a very long sentence which should not be accepted")));
        assert!(check(&rules, &String::from("This is a short sentence")));
    }

    #[test]
    fn test_may_end_with_colon() {
        let mut rules : Rules = Rules {
            may_end_with_colon: false,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("ends with colon:")));

        rules = Rules {
            may_end_with_colon: true,
            ..Default::default()
        };

        assert!(check(&rules, &String::from("ends with colon:")));
    }

    #[test]
    fn test_quote_start_with_letter() {
        let mut rules : Rules = Rules {
            quote_start_with_letter: false,
            needs_letter_start: false,
            ..Default::default()
        };

        assert!(check(&rules, &String::from("\"😊 foo")));

        rules = Rules {
            quote_start_with_letter: true,
            needs_letter_start: false,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("\"😊 foo")));
    }

    #[test]
    fn test_needs_punctuation_end() {
        let mut rules : Rules = Rules {
            needs_punctuation_end: false,
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This has no punctuation")));
        assert!(check(&rules, &String::from("This has punctuation.")));

        rules = Rules {
            needs_punctuation_end: true,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This has no punctuation")));
        assert!(check(&rules, &String::from("This has punctuation.")));
    }

    #[test]
    fn test_needs_letter_start() {
        let mut rules : Rules = Rules {
            needs_letter_start: false,
            ..Default::default()
        };

        assert!(check(&rules, &String::from("?Foo")));
        assert!(check(&rules, &String::from("This has a normal start")));

        rules = Rules {
            needs_letter_start: true,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("?Foo")));
        assert!(check(&rules, &String::from("This has a normal start")));
    }

    #[test]
    fn test_needs_uppercase_start() {
        let mut rules : Rules = Rules {
            needs_uppercase_start: false,
            ..Default::default()
        };

        assert!(check(&rules, &String::from("foo")));
        assert!(check(&rules, &String::from("Foo")));

        rules = Rules {
            needs_uppercase_start: true,
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("foo")));
        assert!(check(&rules, &String::from("Foo")));
    }

    #[test]
    fn test_disallowed_symbols() {
        let rules : Rules = Rules {
            disallowed_symbols: vec![Value::try_from('%').unwrap()],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This has no percentage but other & characters")));
        assert!(!check(&rules, &String::from("This has a %")));
    }

    #[test]
    fn test_allowed_symbols_regex() {
        let rules : Rules = Rules {
            allowed_symbols_regex: String::from("[\u{0020}-\u{005A}]"),
            ..Default::default()
        };

        assert!(check(&rules, &String::from("ONLY UPPERCASE AND SPACE IS ALLOWED")));
        assert!(!check(&rules, &String::from("This is not uppercase")));
    }

    #[test]
    fn test_allowed_symbols_regex_over_disallowed() {
        let rules : Rules = Rules {
            allowed_symbols_regex: String::from("[\u{0020}-\u{005A}]"),
            disallowed_symbols: vec![Value::try_from('O').unwrap()],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("ONLY UPPERCASE AND SPACE IS ALLOWED AND DISALLOWED O IS OKAY")));
    }

    #[test]
    fn test_disallowed_words() {
        let rules : Rules = Rules {
            disallowed_words: ["blerg"].iter().map(|s| (*s).to_string()).collect(),
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This has blerg")));
        assert!(!check(&rules, &String::from("This has a capital bLeRg")));
        assert!(!check(&rules, &String::from("This has many blergs blerg blerg blerg")));
        assert!(!check(&rules, &String::from("Here is a blerg, with comma")));
        assert!(check(&rules, &String::from("This hasn't bl e r g")));

        let rules : Rules = Rules {
            disallowed_words: ["a's"].iter().map(|s| (*s).to_string()).collect(),
            ..Default::default()
        };
        assert!(!check(&rules, &String::from("This has a's")));
    }

    #[test]
    fn test_stem_separator_regex() {
        let rules : Rules = Rules {
            stem_separator_regex: "[']".to_string(),
            disallowed_words: ["Smithsonian", "DC", "Museum"].iter().map(|s| (*s).to_string().to_lowercase()).collect(),
            ..Default::default()
        };

        assert!(check(&rules, &String::from("The Mall has many museums.")));
        assert!(!check(&rules, &String::from("Smithsonian's venues are in the Mall.")));
        assert!(!check(&rules, &String::from("Do you know Smithsonian's African American Museum's location?")));
        assert!(!check(&rules, &String::from("Washington DC's Mall has many museums.")));

        let rules : Rules = Rules {
            disallowed_words: ["Smithsonian", "DC", "Museum"].iter().map(|s| (*s).to_string()).collect(),
            ..Default::default()
        };
        assert!(check(&rules, &String::from("Smithsonian's venues are in DC's Mall - no check for stems.")));
    }

    #[test]
    fn test_broken_whitespace() {
        let rules : Rules = Rules {
            broken_whitespace: vec![Value::try_from("  ").unwrap()],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This has no broken whitespace")));
        assert!(!check(&rules, &String::from("This has  broken whitespace")));
    }

    #[test]
    fn test_abbreviation_patterns() {
        let rules : Rules = Rules {
            abbreviation_patterns: vec![Value::try_from("[A-Z]{2}").unwrap()],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This no two following uppercase letters")));
        assert!(!check(&rules, &String::from("This has two FOllowing uppercase letters")));
    }

    #[test]
    fn test_other_patterns_long_words() {
        let rules : Rules = Rules {
            other_patterns: vec![Value::try_from("\\w{5,50}").unwrap()],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("ये कलाकृतियां खजुराहो मंदिर की कलाकृतियों की याद दिलाती हैं.")));
        assert!(!check(&rules, &String::from("φφδφξασκ")));
        assert!(check(&rules, &String::from("No long test")));
        assert!(!check(&rules, &String::from("Longlong test this is")));
        assert!(!check(&rules, &String::from("This is longlong test")));
        assert!(!check(&rules, &String::from("This is test which is longlong")));
    }

    #[test]
    fn test_uneven_quotes_allowed_default() {
        let rules : Rules = Rules {
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This has \"uneven quotes and it is fine!")));
    }

    #[test]
    fn test_uneven_quotes_allowed() {
        let rules : Rules = Rules {
            even_symbols: vec![],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This has \"uneven quotes and it is fine!")));
        assert!(check(&rules, &String::from("This has (uneven parenthesis and it is fine!")));
    }

    #[test]
    fn test_uneven_quotes_not_allowed() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap(), Value::try_from("(").unwrap()],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This has \"uneven quotes and it is not fine!")));
        assert!(!check(&rules, &String::from("This has (uneven parenthesis and it is not fine!")));
    }

    #[test]
    fn test_uneven_quotes_not_allowed_even() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap()],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This has \"even\" quotes and it is fine!")));
    }

    #[test]
    fn test_uneven_quotes_not_allowed_multiple() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap(), Value::try_from("'").unwrap()],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This has \"uneven quotes' and it is fine!")));
    }

    #[test]
    fn test_uneven_quotes_not_allowed_multiple_one_ok() {
        let rules : Rules = Rules {
            even_symbols: vec![Value::try_from("\"").unwrap(), Value::try_from("'").unwrap()],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This has \"uneven\" quotes' and it is fine!")));
    }

    #[test]
    fn test_matching_quotes_valid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This is „a quote“")));
    }

    #[test]
    fn test_matching_quotes_invalid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This is „a quote")));
    }

    #[test]
    fn test_matching_quotes_valid_multiple() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This is „a quote“ and „another one“")));
    }

    #[test]
    fn test_matching_quotes_invalid_multiple() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("„").unwrap(), Value::try_from("“").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This is „a quote“ and another one“")));
    }

    #[test]
    fn test_matching_bracket_valid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("(").unwrap(), Value::try_from("]").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert!(check(&rules, &String::from("This is (a bracket]")));
    }

    #[test]
    fn test_matching_bracket_invalid() {
        let rules : Rules = Rules {
            matching_symbols: vec![
                Value::try_from([Value::try_from("(").unwrap(), Value::try_from("]").unwrap()]).unwrap()
            ],
            ..Default::default()
        };

        assert!(!check(&rules, &String::from("This is (a bracket")));
    }

    #[test]
    fn test_english() {
        let rules : Rules = load_rules("en");

        assert!(check(&rules, &String::from("This is absolutely valid.")));
        assert!(!check(&rules, &String::from("this is lowercase")));
        assert!(!check(&rules, ""));
        assert!(!check(&rules, &String::from("\"😊")));
        assert!(!check(&rules, &String::from("This ends with:")));
        assert!(!check(&rules, &String::from(" AA ")));
        assert!(!check(&rules, &String::from("This has broken  space")));
        assert!(!check(&rules, &String::from("This as well !")));
        assert!(!check(&rules, &String::from("And this ;")));
        assert!(!check(&rules, &String::from("This is gonna be way way way way way way way way way way too long")));
        assert!(!check(&rules, &String::from("This contains 1 number")));
        assert!(!check(&rules, &String::from("foo\n\nfoo")));
        assert!(!check(&rules, &String::from("foo\\foo")));
        assert!(!check(&rules, &String::from("foo<>")));
        assert!(!check(&rules, &String::from("foo*@")));
        assert!(!check(&rules, &String::from("A.B")));
        assert!(!check(&rules, &String::from("S.T.A.L.K.E.R.")));
    }

    #[test]
    fn test_french() {
        let rules : Rules = load_rules("fr");

        assert!(check(&rules, &String::from("This is absolutely validé.")));
        assert!(!check(&rules, ""));
        assert!(!check(&rules, &String::from("\"😊")));
        assert!(!check(&rules, &String::from("This ends with:")));
        assert!(!check(&rules, &String::from("This does not end with a period")));
        assert!(!check(&rules, &String::from("?This does not start with a letter")));
        assert!(!check(&rules, &String::from("this starts with lowercase")));
        assert!(!check(&rules, &String::from(" AA ")));
        assert!(!check(&rules, &String::from("This has broken  space")));
        assert!(!check(&rules, &String::from("This as well !")));
        assert!(!check(&rules, &String::from("And this ;")));
        assert!(!check(&rules, &String::from("This is gonna be way way way way way way way way way way too long")));
        assert!(!check(&rules, &String::from("Short")));
        assert!(!check(&rules, &String::from("No!!!")));
        assert!(!check(&rules, &String::from("This contains 1 number")));
        assert!(!check(&rules, &String::from("foo\n\nfoo")));
        assert!(!check(&rules, &String::from("foo<>")));
        assert!(!check(&rules, &String::from("foo«")));
        assert!(!check(&rules, &String::from("foo*@")));
        assert!(!check(&rules, &String::from("A.B")));
        assert!(!check(&rules, &String::from("S.T.A.L.K.E.R.")));
        assert!(!check(&rules, &String::from("Some sentence that ends with A.")));
    }

    #[test]
    fn test_german() {
        let rules : Rules = load_rules("de");

        assert!(check(&rules, &String::from("Dies ist ein korrekter Satz.")));
        assert!(!check(&rules, &String::from("Satzzeichen in der Mitte. Wird nicht akzeptiert.")));
        assert!(!check(&rules, &String::from("Satzzeichen in der Mitte? Wird nicht akzeptiert.")));
        assert!(!check(&rules, &String::from("Satzzeichen in der Mitte! Wird nicht akzeptiert.")));
        assert!(!check(&rules, &String::from("Französische Satzzeichen werden ignorierté.")));
        assert!(!check(&rules, &String::from("Andere Satzzeichen wie Åblabla werden auch ignoriert.")));
        assert!(!check(&rules, &String::from("Γεια σας")));
        assert!(!check(&rules, &String::from("Sätze dürfen keine Wörter mit nur einem B Buchstaben haben.")));
        assert!(!check(&rules, &String::from("A auch nicht am Anfang.")));
        assert!(!check(&rules, &String::from("Oder am Ende e.")));
        assert!(!check(&rules, &String::from("AmSi ist eine schwarze Masse, isomorph mit LaSi")));
        assert!(!check(&rules, &String::from("Kein deutsches Wort: ambiguous.")));
        assert!(!check(&rules, &String::from("Zweiter Paragraph im AktG")));
        assert!(!check(&rules, &String::from("Mai in der Domkirche von Badajoz statt.")));
        assert!(!check(&rules, &String::from("Keine Abkürzung mit Umlauten BÄK")));
    }

    #[test]
    fn test_hungarian() {
        let rules : Rules = load_rules("hu");

        assert!(check(&rules, &String::from("A egyik legkiválóbb uralkodójaként tartják számon.")));
        assert!(!check(&rules, &String::from("A BBC Rádió rádiójátékot készített belőle.")));
        assert!(!check(&rules, &String::from("A BD fejlesztései miatt verziószámmal is találkozhatunk.")));
        assert!(!check(&rules, &String::from("A BCS-elmélet más fermionok közti kölcsönhatások leírására is alkalmas.")));
        assert!(!check(&rules, &String::from("A BKV-nál a kocsik elbontásáról döntöttek.")));
        assert!(!check(&rules, &String::from("A BL-ben ötször játszhatott.")));
        assert!(!check(&rules, &String::from("A B-döntőt hat résztvevővel rendezték.")));
        assert!(!check(&rules, &String::from("A -ház egyik legkiválóbb uralkodójaként tartják számon.")));
    }
}

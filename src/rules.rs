use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml::value::Array;
use std::collections::HashSet;
use std::path::Path;

pub fn load_rules(language: &str) -> Rules {
    let file_name = format!("./src/rules/{}.toml", language);
    eprintln!("Loading rules at {:?}", file_name);
    let mut file = File::open(file_name).map_err(|e| format!("{}", e)).unwrap();
    let mut rules_str = String::new();
    file.read_to_string(&mut rules_str)
        .map_err(|e| format!("{}", e)).unwrap();
    let mut rules: Rules = toml::from_str(&rules_str).unwrap();
    eprintln!("Using Rules {:?}", rules);

    let disallowed_file_name = format!("./src/rules/disallowed_words/{}.txt", language);
    let list_exists = Path::new(&disallowed_file_name).exists();
    eprintln!("Using disallowed_word_file = {:?}", list_exists);
    if list_exists {
        let mut file = File::open(disallowed_file_name).map_err(|e| format!("{}", e)).unwrap();
        let mut words_str = String::new();
        file.read_to_string(&mut words_str)
            .map_err(|e| format!("{}", e)).unwrap();
        rules.disallowed_words.extend::<HashSet<String>>(
            words_str
            .split('\n')
            .map(|s| s.trim().to_lowercase())
            .collect()
        );
    }

    rules
}

#[derive(Debug,Deserialize)]
#[serde(default)]
pub struct Rules {
    pub min_trimmed_length: usize,
    pub min_word_count: usize,
    pub max_word_count: usize,
    pub min_characters: usize,
    pub may_end_with_colon: bool,
    pub quote_start_with_letter: bool,
    pub needs_punctuation_end: bool,
    pub needs_uppercase_start: bool,
    pub needs_letter_start: bool,
    pub allowed_symbols_regex: String,
    pub disallowed_symbols: Array,
    pub disallowed_words: HashSet<String>,
    pub broken_whitespace: Array,
    pub abbreviation_patterns: Array,
    pub other_patterns: Array,
    pub replacements: Array,
    pub even_symbols: Array,
    pub matching_symbols: Array,
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            min_trimmed_length: 3,
            min_word_count: 1,
            max_word_count: 14,
            min_characters: 0,
            may_end_with_colon: false,
            quote_start_with_letter: true,
            needs_punctuation_end: false,
            needs_uppercase_start: false,
            needs_letter_start: true,
            allowed_symbols_regex: String::from(""),
            disallowed_symbols: vec![],
            disallowed_words: HashSet::new(),
            broken_whitespace: vec![],
            abbreviation_patterns: vec![],
            other_patterns: vec![],
            replacements: vec![],
            even_symbols: vec![],
            matching_symbols: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_rules() {
        let rules : Rules = Rules {
            ..Default::default()
        };

        assert_eq!(rules.min_trimmed_length, 3);
        assert_eq!(rules.min_word_count, 1);
        assert_eq!(rules.max_word_count, 14);
        assert_eq!(rules.min_characters, 0);
        assert_eq!(rules.may_end_with_colon, false);
        assert_eq!(rules.quote_start_with_letter, true);
        assert_eq!(rules.needs_punctuation_end, false);
        assert_eq!(rules.needs_uppercase_start, false);
        assert_eq!(rules.needs_letter_start, true);
        assert_eq!(rules.allowed_symbols_regex, String::from(""));
        assert_eq!(rules.disallowed_symbols, vec![]);
        assert_eq!(rules.disallowed_words, HashSet::new());
        assert_eq!(rules.broken_whitespace, vec![]);
        assert_eq!(rules.abbreviation_patterns, vec![]);
        assert_eq!(rules.other_patterns, vec![]);
        assert_eq!(rules.replacements, vec![]);
        assert_eq!(rules.even_symbols, vec![]);
        assert_eq!(rules.matching_symbols, vec![]);
    }
}

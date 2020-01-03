use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml::value::Array;
use std::collections::HashSet;
use std::path::Path;

pub fn load_config(language: &str) -> Config {
    let file_name = format!("./src/rules/{}.toml", language);
    eprintln!("Loading config at {:?}", file_name);
    let mut file = File::open(file_name).map_err(|e| format!("{}", e)).unwrap();
    let mut config_str = String::new();
    file.read_to_string(&mut config_str)
        .map_err(|e| format!("{}", e)).unwrap();
    let mut config: Config = toml::from_str(&config_str).unwrap();
    eprintln!("Using Config {:?}", config);

    let disallowed_file_name = format!("./src/rules/disallowed_words/{}.txt", language);
    let list_exists = Path::new(&disallowed_file_name).exists();
    eprintln!("Using disallowed_word_file = {:?}", list_exists);
    if list_exists {
        let mut file = File::open(disallowed_file_name).map_err(|e| format!("{}", e)).unwrap();
        let mut words_str = String::new();
        file.read_to_string(&mut words_str)
            .map_err(|e| format!("{}", e)).unwrap();
        config.disallowed_words.extend::<HashSet<String>>(
            words_str
            .split('\n')
            .map(|s| s.trim().to_lowercase())
            .collect()
        );
    }

    config
}

#[derive(Debug,Deserialize)]
#[serde(default)]
pub struct Config {
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
    pub even_symbols: Array,
    pub require_even_symbols: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
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
            even_symbols: vec![],
            require_even_symbols: false,
        }
    }
}

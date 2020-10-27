#[derive(Clone)]
pub struct Config {
    pub language: String,
    pub directory: String,
    pub max_sentences_per_text: usize,
    pub file_prefix: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            language: String::from("en"),
            directory: String::from(""),
            max_sentences_per_text: 3,
            file_prefix: String::from("wiki_"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_config() {
        let config : Config = Config {
            ..Default::default()
        };

        assert_eq!(config.language, "en");
        assert_eq!(config.directory, "");
        assert_eq!(config.max_sentences_per_text, 3);
        assert_eq!(config.file_prefix, "wiki_");
    }
}

#[derive(Clone)]
pub struct Config {
    pub language: String,
    pub no_check: bool,
    pub directory: String,
    pub max_sentences_per_text: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            language: String::from("en"),
            no_check: false,
            directory: String::from(""),
            max_sentences_per_text: 3,
        }
    }
}

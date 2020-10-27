use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use super::definition::Loader;
use crate::config::Config;

pub struct Wikipedia {
    pub config: Config,
}

impl Wikipedia {
  pub fn new(language: String, directory: String) -> Self {
    let config = Config {
      language,
      directory,
      max_sentences_per_text: 3,
      file_prefix: String::from("wiki_"),
    };

    Self { config }
  }
}

impl Loader for Wikipedia {
  fn get_config(&self) -> &Config {
      &self.config
  }

  fn load(&self, file_name: &PathBuf) -> Result<Vec<String>, String> {
    let mut file = File::open(file_name).map_err(|e| format!("{}", e))?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)
        .map_err(|e| format!("{}", e))?;
    Ok(json_str
        .lines()
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|e| format!("invalid json: {}", e))
                .map(|mut article: Value| {
                    article["text"]
                        .take()
                        .as_str()
                        .map(String::from)
                        .unwrap_or_default()
                })
                .unwrap_or_default()
        })
        .collect())
  }
}

use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

  fn load(&self, file_name: &Path, filtered_titles: &HashSet<String>) -> Result<Vec<String>, String> {
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
                  let text = article["text"]
                    .take()
                    .as_str()
                    .map(String::from)
                    .unwrap_or_default();

                  if filtered_titles.is_empty() {
                    return text
                  }

                  let title_value = article["title"].to_string();
                  let trimmed_title = title_value.trim_matches('"');
                  match filtered_titles.get(&trimmed_title.to_string()) {
                    Some(_) => {
                      eprintln!("Article found in filter list: {:?}", trimmed_title);
                      text
                    },
                    None => String::new(),
                  }
                })
                .unwrap_or_default()
        })
        .collect())
  }
}

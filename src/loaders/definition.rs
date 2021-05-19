use std::path::PathBuf;
use crate::config::Config;

// Each loader can process the files given by file_name as they need to.
// The result should always be a Result<Vec<String>, String>:
//   * A list of Strings, as extracted from the files - this can either be
//     a full paragraph or text of sentences, or simply just one sentence
//     If it's one sentence per entry, rust-punkt will still be used to try to split
//     it and it can't be circumvented. In most cases we will have a text with
//     multiple sentences in it, and it's totally fine - and probably more performant
//     if that is returned as is.
//   * Wrapped in a Result type
// Loaders should preferably be light-weight. If fetching of these sentences is involved,
// we recommend to write a script that can be used before running this scraper. This is
// for example done for the Wikipedia extractor where we use WikiExtractor to get the
// content into shape before running the loader here. This also allows to use existing
// tooling for popular sources and we do not need to reinvent the wheel in this project.

pub trait Loader {
  fn get_config(&self) -> &Config;
  fn load(&self, file_name: &PathBuf) -> Result<Vec<String>, String>;
}

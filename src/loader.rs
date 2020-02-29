use glob::glob;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

// Each loader can process the files given by file_name as they need to.
// The result should always be a Result<Vec<String>, String>:
//   * A list of Strings, as extracted from the files - this can either be
//     a full paragraph or text of sentences, or simply just one sentence
//     If it's one sentence per entry, punkt will still be used to try to split
//     it and it can't be circumvented. In most cases we will have a text with
//     multiple sentences in it, and it's totally fine - and probably more performant
//     if that is returned as is.
//   * Wrapped in a Result type
// Loaders should preferably be light-weight. If fetching of these sentences is involved,
// we recommend to write a script that can be used before running this scraper. This is
// for example done for the Wikipedia extractor where we use WikiExtractor to get the
// content into shape before running the loader here. This also allowes to use existing
// tooling for popular sources and we do not need to reinvent the wheel in this project.

pub fn load_wikiextractor(file_name: &PathBuf) -> Result<Vec<String>, String> {
    let mut file = File::open(file_name).map_err(|e| format!("{}", e))?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)
        .map_err(|e| format!("{}", e))?;
    Ok(json_str
        .lines()
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|e| format!("invalid json{}", e))
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

pub fn load_file_names(dir_name: &str) -> Result<Vec<PathBuf>, String> {
    let chart_path = Path::new(dir_name);
    let glob_path = format!("{}/**/wiki_*", chart_path.to_string_lossy());
    glob(&glob_path)
        .map_err(|e| format!("{}", e))?
        .map(|p| p.map_err(|e| format!("{}", e)))
        .collect::<Result<Vec<PathBuf>, String>>()
}

use crate::errors::*;
use glob::glob;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Article {
    text: String,
}

pub fn load(file_name: &PathBuf) -> Result<Vec<String>> {
    let file = File::open(file_name)?;
    let file = BufReader::new(file);

    file.lines()
        .map(|line| {
            let line = line?;
            let value: Article = serde_json::from_str(&line)?;
            Ok(value.text)
        })
        .collect()
}

pub fn load_file_names(dir_name: &str) -> Result<Vec<PathBuf>> {
    let chart_path = Path::new(dir_name);
    let glob_path = format!("{}/**/wiki_*", chart_path.to_string_lossy());
    glob(&glob_path)?
        .map(|p| p.map_err(Error::from))
        .collect::<Result<Vec<PathBuf>>>()
}

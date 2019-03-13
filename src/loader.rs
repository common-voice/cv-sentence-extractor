use glob::glob;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub fn load(file_name: &PathBuf) -> Result<Vec<String>, String> {
    let mut file = File::open(file_name).map_err(|e| format!("{}", e))?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)
        .map_err(|e| format!("{}", e))?;
    json_str
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
        })
        .collect()
}

pub fn load_file_names(dir_name: &str) -> Result<Vec<PathBuf>, String> {
    let chart_path = Path::new(dir_name);
    let glob_path = format!("{}/**/wiki_*", chart_path.to_string_lossy());
    glob(&glob_path)
        .map_err(|e| format!("{}", e))?
        .map(|p| p.map_err(|e| format!("{}", e)))
        .collect::<Result<Vec<PathBuf>, String>>()
}

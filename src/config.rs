use serde::Deserialize;
use std::fs::File;
use std::io::Read;

pub fn load_config(language: &str) -> Config {
    let file_name = format!("./src/rules/{}.toml", language);
    eprintln!("Loading config at {:?}", file_name);
    // TODO: return default config if files not found..
    let mut file = File::open(file_name).map_err(|e| format!("{}", e)).unwrap();
    let mut config_str = String::new();
    file.read_to_string(&mut config_str)
        .map_err(|e| format!("{}", e)).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap_or_default();
    eprintln!("Using Config {:?}", config);
    config
}

#[derive(Copy,Clone,Debug,Deserialize)]
pub struct Config {
    pub min_trimmed_length: Option<u16>,
    // FIXME: add rest of values..
}

impl Default for Config {
    fn default() -> Config {
        Config {
            min_trimmed_length: None,
        }
    }
}
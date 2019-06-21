use serde::Deserialize;
use std::fs::File;
use std::io::Read;

pub fn load_config(language: &str) -> Config {
    let file_name = format!("rules/{}.toml", language);
    // FIXME: generate default config if no specific config found
    let mut file = File::open(file_name).map_err(|e| format!("{}", e)).unwrap();
    let mut config_str = String::new();
    file.read_to_string(&mut config_str)
        .map_err(|e| format!("{}", e)).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    config
}

#[derive(Deserialize)]
pub struct Config {
    min_trimmed_length: Option<u16>,
    // FIXME: add rest of values..
}
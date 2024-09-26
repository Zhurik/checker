use std;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub token: String,
    pub chat_id: i64,
    pub image: String,
}

pub fn from_file(path: String) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = match std::fs::read_to_string(path) {
        Ok(content) => content.to_owned(),
        Err(e) => return Err(Box::new(e)),
    };

    let config: Config = match toml::from_str(&contents[..]) {
        Ok(config) => config,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(config)
}

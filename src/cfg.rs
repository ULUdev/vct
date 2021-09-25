use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Deserialize)]
/// struct representing a config file for vct
pub struct Config {
    pub langs: Option<Vec<String>>,
    pub iterations: Option<i32>,
    pub rand: Option<bool>,
}

/// load the configuration file
pub fn load_config(path: &str) -> Result<Config, Error> {
    let contents: String = match read_to_string(path) {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };
    let cfg: Config = toml::from_str(contents.as_str()).unwrap();
    Ok(cfg)
}

use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Deserialize, Clone)]
/// struct representing a config file for vct
pub struct Config {
    pub dict: Option<String>,
    pub vocab: Option<String>,
    pub additionals: Option<bool>,
    pub clearlines: Option<bool>,
    pub database: Option<bool>,
    pub dbpath: Option<String>,
}

/// load the configuration file
// TODO: use VctError
pub fn load_config(path: &str) -> Result<Config, Error> {
    let contents: String = match read_to_string(path) {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };
    // TODO: remove unwrap in favor of error handling
    let cfg: Config = toml::from_str(contents.as_str()).unwrap();
    Ok(cfg)
}

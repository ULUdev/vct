use crate::cfg::*;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Vocab {
    name: String,
    meanings: Vec<String>,
}

impl Vocab {
    /// create a new abstracted vocabulary
    /// # Arguments
    /// *`name`: the vocab in the language to learn
    /// *`meanings`: all meanings to learn
    /// # Returns
    /// a new abstracted vocabulary
    pub fn new(name: String, meanings: Vec<String>) -> Vocab {
        Vocab { name, meanings }
    }

    /// parse a String to a vocab
    /// # Arguments
    /// *`string`: the string to parse
    /// # Returns
    /// a new vocabulary wrapped in a `Result`
    pub fn from_string(string: String) -> Result<Vocab, Error> {
        let parts: Vec<&str> = string.as_str().split(';').collect();
        if parts.len() < 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "string has invalid format",
            ));
        }
        let name: String = parts[0].to_string();
        let meanings_str: Vec<&str> = parts[1].split(',').collect();
        let meanings: Vec<String> = meanings_str
            .iter()
            .map(|meaning| meaning.to_lowercase())
            .collect();
        Ok(Vocab::new(name, meanings))
    }

    /// get the meanings of a vocabulary
    /// # Returns
    /// the meanings as a `Vec<String>`
    pub fn get_meanings(&self) -> Vec<String> {
        self.meanings.clone()
    }

    /// get the actual vocab to learn
    /// # Returns
    /// the vocab as a `String`
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub fn load_vocab(config_dir: String, lang: String, conf: Config) -> Result<Vec<Vocab>, Error> {
    let mut dict_dirname: String = format!("{}/dicts", config_dir);
    if conf.dicts != None {
        let dicts = conf.dicts.unwrap();
        for elm in dicts.clone() {
            if elm.starts_with('/') {
                if Path::new(format!("{}/{}", elm, lang.clone()).as_str()).exists() {
                    dict_dirname = elm;
                    break;
                }
            } else if Path::new(format!("{}/{}/{}", config_dir.clone(), elm, lang.clone()).as_str())
                .exists()
            {
                dict_dirname = format!("{}/{}", config_dir, elm);
                break;
            }
        }
    }
    let conf_contents: String = match read_to_string(format!("{}/{}", dict_dirname, lang).as_str())
    {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };
    let mut out: Vec<Vocab> = Vec::new();
    for line in conf_contents.as_str().lines() {
        let new_line: String = line.chars().filter(|x| x != &'\n').collect();
        let voc: Vocab = match Vocab::from_string(new_line) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        out.push(voc);
    }

    Ok(out)
}

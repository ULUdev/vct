use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

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
            return Err(Error::new(ErrorKind::InvalidInput, "string has invalid format"));
        }
        let name: String = parts[0].to_string();
        let meanings_str: Vec<&str> = parts[1].split(',').collect();
        let meanings: Vec<String> = meanings_str.iter().map(|meaning| meaning.to_lowercase()).collect();
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

pub fn load_vocab(config_dir: String, lang: String) -> Result<Vec<Vocab>, Error> {
    let conf_contents: String = match read_to_string(format!("{}/dicts/{}", config_dir, lang).as_str()) {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };
    let mut out: Vec<Vocab> = Vec::new();
    for line in conf_contents.as_str().lines() {
        let new_line: String = line.chars().filter(|x| x != &'\n').collect();
        let voc: Vocab = match Vocab::from_string(new_line){
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        out.push(voc);
    }

    Ok(out)
}

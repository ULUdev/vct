use crate::cfg::*;
use rusqlite::{Connection, Result};
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Vocab {
    name: String,
    meanings: Vec<String>,
    additionals: Option<Vec<String>>,
}

impl Vocab {
    /// create a new abstracted vocabulary
    /// # Arguments
    /// *`name`: the vocab in the language to learn
    /// *`meanings`: all meanings to learn
    /// # Returns
    /// a new abstracted vocabulary
    pub fn new(name: String, meanings: Vec<String>, additionals: Option<Vec<String>>) -> Vocab {
        Vocab {
            name,
            meanings,
            additionals,
        }
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
        if parts.len() > 2 {
            let additionals_str: Vec<&str> = parts[2].split(',').collect();
            let additionals: Vec<String> = additionals_str
                .iter()
                .map(|add| {
                    let add_parts: Vec<&str> = add.split(':').collect();
                    if (add_parts.len() % 2) != 0 {
                        return String::new();
                    }
                    let key = add_parts[0];
                    let val = add_parts[1];
                    format!("{}:{}", key, val)
                })
                .collect();
            return Ok(Vocab::new(name, meanings, Some(additionals)));
        }
        Ok(Vocab::new(name, meanings, None))
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

    /// get the additionals to the vocab
    /// # Returns
    /// the additionals as an `Option<String>`
    pub fn get_additionals(&self) -> Option<Vec<String>> {
        self.additionals.clone()
    }
}

pub fn load_vocab(
    config_dir: String,
    lang: String,
    conf: Config,
    usedb: bool,
) -> Result<Vec<Vocab>, Error> {
    if usedb {
        let mut path: String = match conf.dbpath {
            Some(n) => n,
            None => String::from("db.sqlite"),
        };
        if !path.starts_with('/') {
            path = format!("{}/{}", config_dir, path);
        }
        let db = match Connection::open(path.as_str()) {
            Ok(n) => n,
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "error connecting to database"));
            }
        };
        match db.execute("CREATE TABLE IF NOT EXISTS vocab (lang VARCHAR(256) NOT NULL, name VARCHAR(256) NOT NULL, meanings VARCHAR(256) NOT NULL, additionals VARCHAR(256))", []) {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "error while creating database"));
            }
        }

        let mut sel = match db.prepare(
            format!(
                "SELECT name, meanings, additionals FROM vocab WHERE (lang == {})",
                lang
            )
            .as_str(),
        ) {
            Ok(n) => n,
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "problem with the language provided and the database. Maybe your vocab is in a dict file? Try `--nodb` to disable the database"));
            }
        };
        let vocab_iter = sel.query_map([], |row| {
            let out: Vocab;
            let name: Result<String, rusqlite::Error> = row.get(0);
            let meanings: Result<String, rusqlite::Error> = row.get(1);
            let additionals: Result<String, rusqlite::Error> = row.get(2);
            if let Err(_) = additionals {
                out = match Vocab::from_string(format!("{};{}", name.unwrap(), meanings.unwrap())) {
                    Ok(n) => n,
                    Err(_) => Vocab::new(String::new(), Vec::new(), None),
                };
            } else {
                out = match Vocab::from_string(format!(
                    "{};{};{}",
                    name.unwrap(),
                    meanings.unwrap(),
                    additionals.unwrap()
                )) {
                    Ok(n) => n,
                    Err(_) => Vocab::new(String::new(), Vec::new(), None),
                };
            }
            Ok(out)
        });
        let vocab: Vec<Vocab> = vocab_iter.unwrap().map(|x| x.unwrap()).collect();
        return Ok(vocab);
    }

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

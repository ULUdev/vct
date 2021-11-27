use crate::cfg::*;
use crate::error::*;
use crate::info;
use rusqlite::{params, Connection, Result};
use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

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
    /// `string`: the string to parse
    /// # Returns
    /// a new vocabulary wrapped in a `Result`
    // TODO: use custom vct error
    pub fn from_string(string: String) -> Result<Vocab, VctError> {
        let parts: Vec<&str> = string.as_str().split(';').collect();
        if parts.len() < 2 {
            return Err(VctError::new(
                VctErrorKind::ParsingError,
                "omitting necessary parts of vocabulary",
            ));
        }
        let name: String = parts[0].to_string();
        if name.is_empty() {
            return Err(VctError::new(VctErrorKind::ParsingError, "empty name"));
        }
        let meanings_str: Vec<&str> = parts[1].split(',').collect();
        let meanings: Vec<String> = meanings_str
            .iter()
            .map(|meaning| meaning.to_lowercase())
            .collect();
        if meanings.is_empty() {
            return Err(VctError::new(VctErrorKind::ParsingError, "empty meanings"));
        }
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

// TODO: use VctError
pub fn load_vocab(
    config_dir: String,
    lang: String,
    conf: &Config,
    usedb: bool,
) -> Result<Vec<Vocab>, VctError> {
    if usedb {
        let mut path: String = match &conf.dbpath {
            Some(n) => n.to_string(),
            None => String::from("db.sqlite"),
        };
        if !path.starts_with('/') {
            path = format!("{}/{}", config_dir, path);
        }
        let db = match Connection::open(path.as_str()) {
            Ok(n) => n,
            Err(_) => {
                return Err(VctError::new(
                    VctErrorKind::DatabaseError,
                    "error connecting to database",
                ));
            }
        };
        match db.execute("CREATE TABLE IF NOT EXISTS vocab (lang VARCHAR(256) NOT NULL, name VARCHAR(256) NOT NULL, meanings VARCHAR(256) NOT NULL, additionals VARCHAR(256))", []) {
            Ok(_) => (),
            Err(_) => {
                return Err(VctError::new(VctErrorKind::DatabaseError, "error while creating database"));
            }
        }

        let mut sel = match db.prepare(
            format!(
                "SELECT name, meanings, additionals FROM vocab WHERE (lang == '{}')",
                lang
            )
            .as_str(),
        ) {
            Ok(n) => n,
            Err(_) => {
                return Err(VctError::new(VctErrorKind::DatabaseError, "problem with the language provided and the database. Maybe your vocab is in a dict file? Try `--nodb` to disable the database"));
            }
        };
        let vocab_iter = sel.query_map([], |row| {
            let out: Vocab;
            let name: Result<String, rusqlite::Error> = row.get(0);
            let meanings: Result<String, rusqlite::Error> = row.get(1);
            let additionals: Result<Option<String>, rusqlite::Error> = row.get(2);
            if let Some(adds) = additionals.unwrap() {
                out = match Vocab::from_string(format!(
                    "{};{};{}",
                    name.unwrap(),
                    meanings.unwrap(),
                    adds
                )) {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(rusqlite::Error::ExecuteReturnedResults);
                    }
                };
            } else {
                out = match Vocab::from_string(format!("{};{}", name.unwrap(), meanings.unwrap())) {
                    Ok(n) => n,
                    Err(_) => Vocab::new(String::new(), Vec::new(), None),
                };
            }
            Ok(out)
        });
        let vocab: Vec<Vocab> = vocab_iter.unwrap().map(|x| x.unwrap()).collect();
        Ok(vocab)
    } else {
        let mut dict_dirname: String = format!("{}/dicts", config_dir);
        if conf.dict != None {
            let dict = conf.dict.as_ref().unwrap();
            if dict.is_empty() {
                info::print_info(
                    &btui::Terminal::new(),
                    "dict in config is empty. Ignoring...",
                    info::MessageType::Warning,
                );
            } else if dict.starts_with('/') {
                dict_dirname = dict.clone();
            } else {
                dict_dirname = format!("{}/{}", config_dir, dict.clone());
            }
        }
        let conf_contents: String =
            match read_to_string(format!("{}/{}", dict_dirname, lang).as_str()) {
                Ok(n) => n,
                Err(_) => {
                    return Err(VctError::new(
                        VctErrorKind::FileError,
                        "problem opening dictionary file",
                    ));
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
}

// TODO: add write functionality

pub fn write_vocab(
    file: &str,
    lang: &str,
    vocab: Vocab,
    term: &btui::Terminal,
    db: bool,
) -> Result<()> {
    if db {
        let conn = match Connection::open(file) {
            Ok(n) => n,
            Err(e) => {
                info::print_info(
                    term,
                    format!("error opening connection to database: {}", e),
                    info::MessageType::Error,
                );
                exit(1);
            }
        };

        match conn.execute("CREATE TABLE IF NOT EXISTS vocab (lang VARCHAR(256) NOT NULL, name VARCHAR(256) NOT NULL, meanings VARCHAR(256) NOT NULL, additionals VARCHAR(256))", []) {
            Ok(_) => (),
            Err(e) => {
                info::print_info(term, format!("error preparing database: {}", e), info::MessageType::Error);
                exit(1);
            }
        }

        if let Some(n) = vocab.additionals {
            let mut adds: String = n.iter().map(|x| format!("{},", x)).collect();
            adds = adds[..adds.len() - 1].to_string();
            let mut meanings: String = vocab.meanings.iter().map(|x| format!("{},", x)).collect();
            meanings = meanings[..meanings.len() - 1].to_string();
            match conn.execute(
                "INSERT INTO vocab (lang, name, meanings, additionals) VALUES (?, ?, ?, ?)",
                params![lang, vocab.name, meanings, adds],
            ) {
                Ok(_) => (),
                Err(e) => {
                    info::print_info(
                        term,
                        format!("error inserting into database: {}", e),
                        info::MessageType::Error,
                    );
                    exit(1);
                }
            }
        } else {
            let mut meanings: String = vocab.meanings.iter().map(|x| format!("{},", x)).collect();
            meanings = meanings[..meanings.len() - 1].to_string();
            match conn.execute(
                "INSERT INTO vocab (lang, name, meanings, additionals) VALUES (?, ?, ?, NULL)",
                params![lang, vocab.name, meanings],
            ) {
                Ok(_) => (),
                Err(e) => {
                    info::print_info(
                        term,
                        format!("error inserting into database: {}", e),
                        info::MessageType::Error,
                    );
                    exit(1);
                }
            }
        }

        return Ok(());
    }
    if !Path::new(file).exists() {
        if file.contains('/') {
            let p = Path::new(file);
            let parent = p.parent().unwrap();
            if !parent.exists() {
                match create_dir_all(parent.to_str().unwrap()) {
                    Ok(_) => (),
                    Err(e) => {
                        info::print_info(
                            term,
                            format!("failed creating necessary directories: {}", e),
                            info::MessageType::Error,
                        );
                        exit(1);
                    }
                }
            }
        }
        let mut fhandle = match OpenOptions::new().append(true).open(file) {
            Ok(n) => n,
            Err(e) => {
                info::print_info(
                    term,
                    format!("error opening dictionary file: {}", e),
                    info::MessageType::Error,
                );
                exit(1);
            }
        };
        let mut meanings: String = vocab.meanings.iter().map(|x| format!("{},", x)).collect();
        meanings = meanings[..meanings.len() - 1].to_string();
        if let Some(adds) = vocab.additionals {
            let mut additionals: String = adds.iter().map(|x| format!("{},", x)).collect();
            additionals = additionals[..additionals.len() - 1].to_string();
            match fhandle.write_all(
                format!("{};{};{}", vocab.name, meanings, additionals)
                    .as_str()
                    .as_bytes(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    info::print_info(
                        term,
                        format!("error writing to file: {}", e),
                        info::MessageType::Error,
                    );
                    exit(1);
                }
            }
        } else {
            match fhandle.write_all(format!("{};{}", vocab.name, meanings).as_str().as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    info::print_info(
                        term,
                        format!("error writing to file: {}", e),
                        info::MessageType::Error,
                    );
                    exit(1);
                }
            }
        }
    }

    Ok(())
}
// TODO: add a function to load all vocabulary from dictionaries

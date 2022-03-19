use crate::cfg::*;
use crate::dict::Vocab;
use crate::error::*;
use rusqlite::{Connection, Result};
use std::fs::read_to_string;

/// walk a directory recursively and return all the files found
pub fn walk_through_dir(path: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(n) = std::fs::read_dir(path) {
        for result in n {
            if let Ok(entry) = result {
                if let Ok(ftype) = entry.file_type() {
                    if ftype.is_file() {
                        out.push(entry.path().into_os_string().into_string().unwrap());
                    } else if ftype.is_dir() {
                        for i in
                            walk_through_dir(entry.path().into_os_string().into_string().unwrap())
                        {
                            out.push(i);
                        }
                    }
                }
            }
        }
    }
    out
}

pub fn query(
    query_string: String,
    config_dir: String,
    conf: &Config,
    usedb: bool,
) -> Result<Vec<Vocab>, VctError> {
    let mut found: Vec<Vocab> = Vec::new();
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
        let mut sel = match db.prepare("SELECT name, meanings, additionals FROM vocab") {
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
            if name.as_ref().unwrap().as_str().contains(&query_string)
                || meanings.as_ref().unwrap().as_str().contains(&query_string)
            {
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
                    out = match Vocab::from_string(format!(
                        "{};{}",
                        name.unwrap(),
                        meanings.unwrap()
                    )) {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(rusqlite::Error::ExecuteReturnedResults);
                        }
                    };
                }
                return Ok(out);
            } else {
                return Err(rusqlite::Error::ExecuteReturnedResults);
            }
        });
        let vocab: Vec<Vocab> = vocab_iter
            .unwrap()
            .filter(|x| !x.is_err())
            .map(|x| x.unwrap())
            .collect();
        for i in vocab {
            found.push(i);
        }
    }
    let dir: String = config_dir;
    let paths: Vec<String> = walk_through_dir(format!(
        "{}/{}",
        dir,
        conf.dict.as_ref().unwrap_or(&String::from("dicts"))
    ));
    for path in paths {
        let contents: String = match read_to_string(path.as_str()) {
            Ok(n) => n,
            Err(_) => {
                continue;
            }
        };
        for line in contents
            .as_str()
            .lines()
            .filter(|line| line.contains(&query_string))
        {
            if let Ok(n) = Vocab::from_string(line.chars().filter(|x| x != &'\n').collect()) {
                found.push(n);
            }
        }
    }
    Ok(found)
}

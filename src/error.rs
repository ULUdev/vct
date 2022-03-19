use std::error::Error;
use std::fmt;

/// enum representing an error that can occur in vct
#[derive(Debug)]
#[allow(dead_code)]
pub enum VctErrorKind {
    /// an error that occured while parsing user input or a file
    ParsingError,
    ParamError,
    DatabaseError,
    FileError,
}

/// struct representing an error occuring within vct
#[derive(Debug)]
pub struct VctError {
    msg: String,
}

impl VctError {
    pub fn new(kind: VctErrorKind, msg: &str) -> VctError {
        let message: String = match kind {
            VctErrorKind::ParsingError => format!("parsing error: {}", msg),
            VctErrorKind::ParamError => format!("parameter error: {}", msg),
            VctErrorKind::DatabaseError => format!("database error: {}", msg),
            VctErrorKind::FileError => format!("file error: {}", msg),
        };
        VctError { msg: message }
    }
}

impl Error for VctError {}

impl fmt::Display for VctError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

use btui::{effects::*, print::*};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
/// struct representing vocabulary
pub struct Vocab {
    vocab: String,
    meaning: Vec<String>,
}

impl Vocab {
    /// create a new vocab
    pub fn new(vocab: String, meaning: Vec<String>) -> Vocab {
        Vocab { vocab, meaning }
    }

    pub fn get_vocab(&self) -> String {
        self.vocab.clone()
    }

    pub fn get_meaning(&self) -> Vec<String> {
        self.meaning.clone()
    }

    pub fn set_vocab(&mut self, vocab: String) {
        self.vocab = vocab;
    }

    pub fn set_meaning(&mut self, meaning: Vec<String>) {
        self.meaning = meaning;
    }
}

pub fn vocab_from_file(filename: &str) -> Result<Vec<Vocab>, std::io::Error> {
    let contents: String = match read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            return Err(e);
        }
    };
    let mut out: Vec<Vocab> = Vec::new();
    let mut idx: usize = 1;
    for line in contents.as_str().lines() {
        if line.is_empty() {
            continue;
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                eprintln!(
                    "{}vct: error in dict {} at line {}: no correct vocab provided{}",
                    fg(Color::Red),
                    filename,
                    idx,
                    sp(Special::Reset)
                );
            } else {
                let mut left: Vec<String> = Vec::new();
                for i in parts[1..].iter() {
                    left.push(i.to_string());
                }
                out.push(Vocab::new(parts[0].to_string(), left));
            }
        }
        idx += 1;
    }

    Ok(out)
}

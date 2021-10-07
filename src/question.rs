use btui::pbar::ProgressBar;
use btui::{effects::*, print::*};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{stdin, stdout, Write};
use std::process::exit;

pub fn question_vocab(lang: String, vocab: Vec<crate::dict::Vocab>) -> usize {
    println!(
        "{}You will be learning {} {} vocabularies{}",
        fg(Color::Green),
        lang,
        vocab.len(),
        sp(Special::Reset)
    );
    let mut progress = 0usize;
    let mut done: Vec<&crate::dict::Vocab> = Vec::new();
    while done.len() != vocab.len() {
        let mut cur_vocab = match vocab.choose(&mut thread_rng()) {
            Some(n) => n,
            None => {
                return 0usize;
            }
        };
        while done.contains(&cur_vocab) {
            cur_vocab = match vocab.choose(&mut thread_rng()) {
                Some(n) => n,
                None => {
                    return 0usize;
                }
            };
        }
        let meanings = cur_vocab.get_meanings().len();
        let mut meanings_done = 0usize;
        let mut so = stdout();
        print!(
            "{}what does {} mean? ({}/{}): {}",
            fg(Color::Green),
            cur_vocab.get_name(),
            meanings_done,
            meanings,
            sp(Special::Reset)
        );
        so.flush();
        let mut input: String = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}vct: error: {}{}", fg(Color::Red), e, sp(Special::Reset));
                exit(1);
            }
        }
        input = input.as_str().chars().filter(|x| x != &'\n').collect();
        let captured: String = input.as_str().to_lowercase();
        if cur_vocab.get_meanings().contains(&captured) {
            println!("{}correct!{}", fg(Color::Green), sp(Special::Reset));
            progress += 1;
        } else {
            println!("{}wrong! {}{}{:?}{} would have been right{}", fg(Color::Red), fg(Color::White), sp(Special::Bold), cur_vocab.get_meanings(), fg(Color::Red), sp(Special::Reset));
        }
        done.push(cur_vocab);
    }

    progress
}

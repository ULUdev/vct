use btui::linux::console::*;
use btui::{effects::*, print::*};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{stdin, stdout, Write};
use std::process::exit;

pub fn question_vocab(
    lang: String,
    vocab: Vec<crate::dict::Vocab>,
    amount: String,
    adds: bool,
    clearlines: bool,
) -> (usize, usize) {
    println!(
        "{}You will be learning {} {} vocabularies{}",
        fg(Color::Green),
        vocab.len(),
        lang,
        sp(Special::Reset)
    );
    let mut progress: usize = 0;
    let mut add_progress: usize = 0;
    let mut done: Vec<&crate::dict::Vocab> = Vec::new();
    while done.len() != vocab.len() {
        let mut cur_vocab = match vocab.choose(&mut thread_rng()) {
            Some(n) => n,
            None => {
                return (0usize, 0usize);
            }
        };
        while done.contains(&cur_vocab) {
            cur_vocab = match vocab.choose(&mut thread_rng()) {
                Some(n) => n,
                None => {
                    return (0usize, 0usize);
                }
            };
        }
        let meanings = cur_vocab.get_meanings().len();
        let mut meanings_done_count = 0usize;
        let mut meanings_done: Vec<String> = Vec::new();
        let mut so = stdout();
        while meanings != meanings_done_count {
            print!(
                "{}what does '{}' mean? ({}/{})? > {}",
                fg(Color::White),
                cur_vocab.get_name(),
                meanings_done_count,
                meanings,
                sp(Special::Reset)
            );
            let _ = match so.flush() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!(
                        "{}vct: error when flushing stdout: {}{}",
                        fg(Color::Red),
                        e,
                        sp(Special::Reset)
                    );
                }
            };
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

            // clear the screen if needed
            if clearlines {
                dc_sequence(DisplayControl::ClearLine);
                cc_sequence(CursorControl::Up(1));
                dc_sequence(DisplayControl::ClearLine);
                cc_sequence(CursorControl::Up(1));
                dc_sequence(DisplayControl::ClearLine);
                cc_sequence(CursorControl::Col(1));
            }
            if meanings_done.contains(&captured) {
                println!("{}already used{}", fg(Color::Red), sp(Special::Reset));
                continue;
            }
            if cur_vocab.get_meanings().contains(&captured) {
                println!("{}correct!{}", fg(Color::Green), sp(Special::Reset));
                if amount == *"one" {
                    progress += 1;
                    break;
                }
                meanings_done_count += 1;
                meanings_done.push(captured);
            } else {
                let mut correct_meanings_string: String = cur_vocab.get_meanings()[0].clone();
                for meaning in cur_vocab.get_meanings()[1..].to_vec() {
                    correct_meanings_string.push_str(format!(", {}", meaning).as_str());
                }
                println!(
                    "{}wrong! {}{}{:?}{} would have been right{}",
                    fg(Color::Red),
                    fg(Color::White),
                    sp(Special::Bold),
                    correct_meanings_string,
                    fg(Color::Red),
                    sp(Special::Reset)
                );
                break;
            }
        }
        if meanings == meanings_done_count {
            progress += 1;
        }
        if !adds {
            done.push(cur_vocab);
            continue;
        }
        if let Some(adds) = cur_vocab.get_additionals() {
            let mut adds_done: Vec<String> = Vec::new();
            let mut idx: usize = 0;
            while adds_done.len() < adds.len() {
                let key = adds[idx].split(':').next().unwrap();
                let value = adds[idx].split(':').nth(1).unwrap();
                print!(
                    "{}(additional) what is '{}' of '{}'? > {}",
                    fg(Color::White),
                    key,
                    cur_vocab.get_name(),
                    sp(Special::Reset)
                );
                let _ = match so.flush() {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!(
                            "{}vct: error when flushing stdout: {}{}",
                            fg(Color::Red),
                            e,
                            sp(Special::Reset)
                        );
                    }
                };
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
                //
                // clear the screen if needed
                if clearlines {
                    dc_sequence(DisplayControl::ClearLine);
                    cc_sequence(CursorControl::Up(1));
                    dc_sequence(DisplayControl::ClearLine);
                    cc_sequence(CursorControl::Up(1));
                    dc_sequence(DisplayControl::ClearLine);
                    cc_sequence(CursorControl::Col(1));
                }

                if captured == value {
                    adds_done.push(adds[idx].clone());
                    add_progress += 1;
                    println!("{}correct!{}", fg(Color::Green), sp(Special::Reset));
                } else {
                    println!(
                        "{}Wrong! {}{}'{}'{}{} would have been right.{}",
                        fg(Color::Red),
                        fg(Color::White),
                        sp(Special::Bold),
                        value,
                        sp(Special::Reset),
                        fg(Color::Red),
                        sp(Special::Reset)
                    );
                    adds_done.push(adds[idx].clone());
                }
                idx += 1;
            }
        }
        done.push(cur_vocab);
    }

    (progress, add_progress)
}

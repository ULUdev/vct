use btui::Terminal;
use btui::{effects::*, print::*};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{stdout, Write};
use std::process::exit;

pub fn question_vocab(
    lang: String,
    vocab: Vec<crate::dict::Vocab>,
    amount: String,
    adds: bool,
    clearlines: bool,
) -> (usize, usize) {
    let term: Terminal = Terminal::default();
    let mut progress: usize = 0;
    let mut add_progress: usize = 0;
    let mut done: Vec<&crate::dict::Vocab> = Vec::new();
    term.println(format!(
        "{}You will be learning {} {} vocabularies{}",
        fg(Color::Green),
        vocab.len(),
        lang,
        sp(Special::Reset)
    ))
    .unwrap();
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
            term.print(format!(
                "{}what does '{}' mean? ({}/{})? > {}",
                fg(Color::White),
                cur_vocab.get_name(),
                meanings_done_count,
                meanings,
                sp(Special::Reset)
            ))
            .unwrap();
            let _ = match so.flush() {
                Ok(_) => (),
                Err(e) => {
                    term.eprintln(format!(
                        "{}vct: error when flushing stdout: {}{}",
                        fg(Color::Red),
                        e,
                        sp(Special::Reset)
                    ))
                    .unwrap();
                }
            };
            let input: String = match term.read_line_trimmed() {
                Ok(n) => n,
                Err(e) => {
                    term.eprintln(format!(
                        "{}vct: error: {}{}",
                        fg(Color::Red),
                        e,
                        sp(Special::Reset)
                    ))
                    .unwrap();
                    exit(1);
                }
            };
            let captured: String = input.as_str().to_lowercase();

            // clear the screen if needed
            if clearlines {
                term.move_cursor(0, -1).unwrap();
                term.clear_line().unwrap();
                term.move_cursor(0, -1).unwrap();
                //term.clear_line().unwrap();
                term.clear_line().unwrap();
                term.set_cursor_x(1).unwrap();
            }
            if meanings_done.contains(&captured) {
                term.println(format!(
                    "{}already used{}",
                    fg(Color::Red),
                    sp(Special::Reset)
                ))
                .unwrap();
                continue;
            }
            if cur_vocab.get_meanings().contains(&captured) {
                term.println(format!(
                    "{}correct!{}",
                    fg(Color::Green),
                    sp(Special::Reset)
                ))
                .unwrap();
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
                term.println(format!(
                    "{}wrong! {}{}{:?}{} would have been right{}",
                    fg(Color::Red),
                    fg(Color::White),
                    sp(Special::Bold),
                    correct_meanings_string,
                    fg(Color::Red),
                    sp(Special::Reset)
                ))
                .unwrap();
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
                term.print(format!(
                    "{}(additional) what is '{}' of '{}'? > {}",
                    fg(Color::White),
                    key,
                    cur_vocab.get_name(),
                    sp(Special::Reset)
                ))
                .unwrap();
                let _ = match so.flush() {
                    Ok(_) => (),
                    Err(e) => {
                        term.eprintln(format!(
                            "{}vct: error when flushing stdout: {}{}",
                            fg(Color::Red),
                            e,
                            sp(Special::Reset)
                        ))
                        .unwrap();
                    }
                };
                let input: String = match term.read_line_trimmed() {
                    Ok(n) => n,
                    Err(e) => {
                        term.eprintln(format!(
                            "{}vct: error: {}{}",
                            fg(Color::Red),
                            e,
                            sp(Special::Reset)
                        ))
                        .unwrap();
                        exit(1);
                    }
                };
                let captured: String = input.as_str().to_lowercase();
                //
                // clear the screen if needed
                if clearlines {
                    term.clear_line().unwrap();
                    term.move_cursor(0, -1).unwrap();
                    term.clear_line().unwrap();
                    term.move_cursor(0, -1).unwrap();
                    term.clear_line().unwrap();
                    term.set_cursor_x(1).unwrap();
                }

                if captured == value {
                    adds_done.push(adds[idx].clone());
                    add_progress += 1;
                    term.println(format!(
                        "{}correct!{}",
                        fg(Color::Green),
                        sp(Special::Reset)
                    ))
                    .unwrap();
                } else {
                    term.println(format!(
                        "{}Wrong! {}{}'{}'{}{} would have been right.{}",
                        fg(Color::Red),
                        fg(Color::White),
                        sp(Special::Bold),
                        value,
                        sp(Special::Reset),
                        fg(Color::Red),
                        sp(Special::Reset)
                    ))
                    .unwrap();
                    adds_done.push(adds[idx].clone());
                }
                idx += 1;
            }
        }
        done.push(cur_vocab);
    }

    (progress, add_progress)
}

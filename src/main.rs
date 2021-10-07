use btui::{effects::*, print::*, pbar::ProgressBar};
use rand::seq::SliceRandom;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::exit;

mod args;
mod cfg;
mod dict;
mod question;

use args::{load_params, Params, HELP_STR, VERSION_STR};
use cfg::*;
use dict::*;

fn main() {
    let params: Params = load_params();
    let mut quit: bool = false;
    if params.help {
        eprintln!("{}", HELP_STR);
        quit = false;
    }
    if params.version {
        eprintln!("{}", VERSION_STR);
        quit = false
    }

    if quit {
        exit(0);
    }

    if !Path::new(params.config_path.as_str()).exists() {
        match create_dir_all(params.config_dir.as_str()) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "{}vct: couldn't create config dir{}",
                    fg(Color::Red),
                    sp(Special::Reset)
                );
                exit(1);
            }
        }
        match File::create(params.config_path.as_str()) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "{}vct: failed creating config file{}",
                    fg(Color::Red),
                    sp(Special::Reset)
                );
                exit(1);
            }
        }
    }
    let conf: Config = match load_config(params.config_path.as_str()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "{red}vct: error loading config: {bold}{err}{reset}",
                red = fg(Color::Red),
                bold = sp(Special::Bold),
                err = e,
                reset = sp(Special::Reset)
            );
            exit(1);
        }
    };

    if params.lang == String::new() {
        exit(0);
    }
    let vocab = match load_vocab(params.config_dir, params.lang.clone()) {
        Ok(n) => n,
        Err(e) => {
            eprintln!(
                "{}vct: error while parsing vocabulary dictionary: {}{}",
                fg(Color::Red),
                e,
                sp(Special::Reset)
            );
            exit(1);
        }
    };
    let result: usize = question::question_vocab(params.lang.clone(), vocab.clone());
    let vocab_total: usize = vocab.clone().len();
    let total: usize = ((result/vocab_total)*100usize);
    let total: u8 = format!("{}", total).parse().unwrap();
    let mut bar = ProgressBar::new("result", ' ', '#');
    bar.set_progress(total);
    println!("you had {} out of {} correct", result, vocab_total);
    println!("{}", bar.render());

    exit(0);
}

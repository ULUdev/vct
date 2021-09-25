use btui::{effects::*, print::*};
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::exit;
use rand::seq::SliceRandom;
use rand::thread_rng();

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
    let vocab: Vec<Vocab> =
        match vocab_from_file(format!("{}/{}", params.config_dir, params.lang).as_str()) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}vct: error: {}{}", fg(Color::Red), e, sp(Special::Reset));
                exit(1);
            }
        };

    exit(0);
}

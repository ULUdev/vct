use btui::{effects::*, pbar::ExtProgressBar, print::*};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

mod args;
mod cfg;
mod dict;
mod question;

use args::{load_params, Params};
use cfg::*;
use dict::*;

fn main() {
    let params: Params = load_params();
    let mut dict_dirname: String = format!("{}/dicts", params.config_dir);

    if params.quit {
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
    if !Path::new(format!("{}/dicts", params.config_path).as_str()).exists() {
        match create_dir_all(format!("{}/dicts", params.config_dir).as_str()) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "{}vct: couldn't create dicts dir{}",
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
    if params.dict != String::new() {
        let parts: Vec<String> = params
            .dict
            .as_str()
            .split(';')
            .map(|x| x.to_string())
            .collect();
        let dict_fname: String = parts[0].clone();
        let name: String = parts[1].clone();
        let meanings: String = parts[2].clone();
        if conf.dicts != None {
            let dicts = conf.clone().dicts.unwrap();
            for elm in dicts.clone() {
                if elm.starts_with('/') {
                    if Path::new(elm.to_string().as_str()).exists() {
                        dict_dirname = elm;
                        break;
                    }
                } else if Path::new(format!("{}/{}", params.config_dir.clone(), elm).as_str())
                    .exists()
                {
                    dict_dirname = format!("{}/{}", params.config_dir, elm);
                    break;
                }
            }
        }
        if !Path::new(format!("{}/{}", dict_dirname, dict_fname).as_str()).exists() {
            let _ = match File::create(format!("{}/{}", dict_dirname, dict_fname).as_str()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!(
                        "{}vct: error creating file: {}{}",
                        fg(Color::Red),
                        e,
                        sp(Special::Reset)
                    );
                    exit(1);
                }
            };
        }
        let mut file = match OpenOptions::new()
            .append(true)
            .open(format!("{}/{}", dict_dirname, dict_fname).as_str())
        {
            Ok(n) => n,
            Err(e) => {
                eprintln!(
                    "{}vct: error opening dictionary: {}{}",
                    fg(Color::Red),
                    e,
                    sp(Special::Reset)
                );
                exit(1);
            }
        };
        match file.write_all(format!("{};{}\n", name, meanings).as_str().as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "{}vct: error writing to file: {:?}{}",
                    fg(Color::Red),
                    e,
                    sp(Special::Reset)
                );
                exit(1);
            }
        }
    }

    if params.lang == String::new() {
        exit(0);
    }
    let vocab = match load_vocab(params.config_dir, params.lang.clone(), conf) {
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
    let result: f32 = question::question_vocab(params.lang, vocab.clone()) as f32;
    let vocab_total: f32 = vocab.len() as f32;
    let total: u8 = ((result / vocab_total) * 100.0) as u8;
    let mut bar = ExtProgressBar::new("[=> ]", "result");
    bar.set_progress(total);
    println!("\nyou had {} out of {} correct", result, vocab_total);
    println!("{}\n", bar.render());

    exit(0);
}

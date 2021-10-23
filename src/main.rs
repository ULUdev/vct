use btui::linux::console::*;
use btui::{effects::*, pbar::ExtProgressBar, print::*};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

mod args;
mod cfg;
mod dict;
mod pretty_print;
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
        let mut parts = params.dict.as_str().split(';').map(|x| x.to_string());
        let dict_fname: String = parts.next().unwrap();
        let name: String = parts.next().unwrap();
        let mut meanings: String = parts.next().unwrap();
        if let Some(n) = parts.next() {
            meanings.push_str(format!(";{}", n).as_str());
        }
        if conf.dicts != None {
            let dicts = conf.clone().dicts.unwrap();
            if dicts.len() > 0 {
                if dicts[0].clone().starts_with('/') {
                    dict_dirname = dicts[0].clone();
                } else {
                    dict_dirname = format!("{}/{}", params.config_dir, dicts[0].clone());
                }
            }
        }
        if let Some(n) = Path::new(&dict_fname).parent() {
            if !n.exists() {
                let mut parent_path: String = n.clone().to_str().unwrap().to_string();
                if parent_path.starts_with('/') {
                    eprintln!("{}vct: warning: path cannot start with a '/'. Ignoring...{}", fg(Color::Yellow), sp(Special::Reset));
                    parent_path = parent_path.as_str()[1..].to_string();
                }
                parent_path = format!("{}/{}", dict_dirname.clone(), parent_path);
                match create_dir_all(parent_path.as_str()) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!(
                            "{}vct: error: couldn't create required directories: {}{}",
                            fg(Color::Red),
                            e,
                            sp(Special::Reset)
                        );
                    }
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

    if let Some(n) = params.pretprin {
        let voc: Vec<Vocab> = match load_vocab(params.config_dir.clone(), n, conf.clone()) {
            Ok(p) => p,
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
        println!("{}", pretty_print::pretty_print(voc));
        exit(0);
    }

    if params.lang == String::new() {
        exit(0);
    }
    let vocab = match load_vocab(params.config_dir, params.lang.clone(), conf.clone()) {
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
    let amount: String = match params.vocab.as_str() {
        "all" => String::from("all"),
        "one" => String::from("one"),
        _ => match conf.vocab {
            Some(n) => match n.as_str() {
                "all" => String::from("all"),
                "one" => String::from("one"),
                _ => String::from("one"),
            },
            None => String::from("one"),
        },
    };
    let adds: bool = match params.adds {
        Some(n) => n,
        None => conf.additionals.unwrap_or(true),
    };
    let clearlines: bool = match params.clearlines {
        Some(n) => n,
        None => conf.clearlines.unwrap_or(false),
    };
    let (normal, add) =
        question::question_vocab(params.lang, vocab.clone(), amount, adds, clearlines);

    // if clearlines is enabled clear the line above
    if clearlines {
        cc_sequence(CursorControl::Up(1));
        dc_sequence(DisplayControl::ClearLine);
        cc_sequence(CursorControl::Col(1));
    }

    let result: f32 = normal as f32;
    let vocab_total: f32 = vocab.len() as f32;
    let total: u8 = ((result / vocab_total) * 100.0) as u8;
    let mut norm_bar = ExtProgressBar::new("[=> ]", "result");
    norm_bar.set_progress(total);
    println!("\nyou had {} out of {} correct", result, vocab_total);
    println!("{}\n", norm_bar.render());

    if !adds {
        exit(0);
    }
    let add_result: f32 = add as f32;
    let mut add_total: f32 = 0.0;
    for i in vocab {
        if let Some(n) = i.get_additionals() {
            add_total += n.len() as f32;
        }
    }
    let add_score: u8 = ((add_result / add_total) * 100.0) as u8;
    let mut add_bar = ExtProgressBar::new("[=> ]", "result");
    add_bar.set_progress(add_score);
    println!(
        "\n(additional) you had {} out of {} correct",
        add_result, add_total
    );
    println!("\n{}", add_bar.render());

    exit(0);
}

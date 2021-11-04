use btui::Terminal;
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
    let term: Terminal = Terminal::new();

    if params.quit {
        exit(0);
    }

    if !Path::new(params.config_path.as_str()).exists() {
        match create_dir_all(params.config_dir.as_str()) {
            Ok(_) => (),
            Err(_) => {
                term.eprintln(format!(
                    "{}vct: couldn't create config dir{}",
                    fg(Color::Red),
                    sp(Special::Reset)
                ))
                .unwrap();
                exit(1);
            }
        }
        match File::create(params.config_path.as_str()) {
            Ok(_) => (),
            Err(_) => {
                term.eprintln(format!(
                    "{}vct: failed creating config file{}",
                    fg(Color::Red),
                    sp(Special::Reset)
                ))
                .unwrap();
                exit(1);
            }
        }
    }
    if !Path::new(format!("{}/dicts", params.config_path).as_str()).exists() {
        match create_dir_all(format!("{}/dicts", params.config_dir).as_str()) {
            Ok(_) => (),
            Err(_) => {
                term.eprintln(format!(
                    "{}vct: couldn't create dicts dir{}",
                    fg(Color::Red),
                    sp(Special::Reset)
                ))
                .unwrap();
                exit(1);
            }
        }
    }
    let conf: Config = match load_config(params.config_path.as_str()) {
        Ok(c) => c,
        Err(e) => {
            term.eprintln(format!(
                "{red}vct: error loading config: {bold}{err}{reset}",
                red = fg(Color::Red),
                bold = sp(Special::Bold),
                err = e,
                reset = sp(Special::Reset)
            ))
            .unwrap();
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
            if !dicts.is_empty() {
                if dicts[0].clone().starts_with('/') {
                    dict_dirname = dicts[0].clone();
                } else {
                    dict_dirname = format!("{}/{}", params.config_dir, dicts[0].clone());
                }
            }
        }
        if let Some(n) = Path::new(&dict_fname).parent() {
            if !n.exists() {
                let mut parent_path: String = (*n).to_str().unwrap().to_string();
                if parent_path.starts_with('/') {
                    term.eprintln(format!(
                        "{}vct: warning: path cannot start with a '/'. Ignoring...{}",
                        fg(Color::Yellow),
                        sp(Special::Reset)
                    ))
                    .unwrap();
                    parent_path = parent_path.as_str()[1..].to_string();
                }
                parent_path = format!("{}/{}", dict_dirname, parent_path);
                match create_dir_all(parent_path.as_str()) {
                    Ok(_) => (),
                    Err(e) => {
                        term.eprintln(format!(
                            "{}vct: error: couldn't create required directories: {}{}",
                            fg(Color::Red),
                            e,
                            sp(Special::Reset)
                        ))
                        .unwrap();
                    }
                }
            }
        }
        if !Path::new(format!("{}/{}", dict_dirname, dict_fname).as_str()).exists() {
            let _ = match File::create(format!("{}/{}", dict_dirname, dict_fname).as_str()) {
                Ok(_) => (),
                Err(e) => {
                    term.eprintln(format!(
                        "{}vct: error creating file: {}{}",
                        fg(Color::Red),
                        e,
                        sp(Special::Reset)
                    ))
                    .unwrap();
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
                term.eprintln(format!(
                    "{}vct: error opening dictionary: {}{}",
                    fg(Color::Red),
                    e,
                    sp(Special::Reset)
                ))
                .unwrap();
                exit(1);
            }
        };
        match file.write_all(format!("{};{}\n", name, meanings).as_str().as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                term.eprintln(format!(
                    "{}vct: error writing to file: {:?}{}",
                    fg(Color::Red),
                    e,
                    sp(Special::Reset)
                ))
                .unwrap();
                exit(1);
            }
        }
    }

    let usedb: bool = match params.usedb {
        Some(n) => n,
        None => conf.database.unwrap_or(false),
    };
    if let Some(n) = params.pretprin {
        let voc: Vec<Vocab> = match load_vocab(params.config_dir.clone(), n, conf.clone(), usedb) {
            Ok(p) => p,
            Err(e) => {
                term.eprintln(format!(
                    "{}vct: error while parsing vocabulary dictionary: {}{}",
                    fg(Color::Red),
                    e,
                    sp(Special::Reset)
                ))
                .unwrap();
                exit(1);
            }
        };
        term.println(pretty_print::pretty_print(voc)).unwrap();
        exit(0);
    }

    if params.lang == String::new() {
        exit(0);
    }
    let vocab = match load_vocab(params.config_dir.clone(), params.lang.clone(), conf.clone(), usedb) {
        Ok(n) => n,
        Err(e) => {
            term.eprintln(format!(
                "{}vct: error while parsing vocabulary dictionary: {}{}",
                fg(Color::Red),
                e,
                sp(Special::Reset)
            ))
            .unwrap();
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
        term.move_cursor(0, -1).unwrap();
        term.clear_line().unwrap();
        term.set_cursor_x(1).unwrap();
    }

    let result: f32 = normal as f32;
    let vocab_total: f32 = vocab.len() as f32;
    let total: f32 = ((result / vocab_total) * 100.0) as f32;
    let mut norm_bar = ExtProgressBar::new("[=> ]", "result");
    norm_bar.set_progress(total);
    term.println(format!(
        "\nyou had {} out of {} correct",
        result, vocab_total
    ))
    .unwrap();
    term.println(format!("{}\n", norm_bar.render())).unwrap();

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
    let add_score: f32 = ((add_result / add_total) * 100.0) as f32;
    let mut add_bar = ExtProgressBar::new("[=> ]", "result");
    add_bar.set_progress(add_score);
    term.println(format!(
        "\n(additional) you had {} out of {} correct",
        add_result, add_total
    ))
    .unwrap();
    term.println(format!("\n{}", add_bar.render())).unwrap();

    exit(0);
}

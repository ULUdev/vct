use btui::pbar::ExtProgressBar;
use btui::Terminal;
use std::fs::{create_dir_all, File};
// use std::io::Write;
use std::path::Path;
use std::process::exit;

mod args;
mod cfg;
mod dict;
mod error;
mod info;
mod pretty_print;
mod query;
mod question;

use args::{load_params, Params};
use cfg::*;
use dict::*;

fn main() {
    let params: Params = load_params();
    // let mut dict_dirname: String = format!("{}/dicts", params.config_dir);
    let term: Terminal = Terminal::new();

    if params.quit {
        exit(0);
    }

    if !Path::new(params.config_path.as_str()).exists() {
        match create_dir_all(params.config_dir.as_str()) {
            Ok(_) => (),
            Err(_) => {
                info::print_info(
                    &term,
                    "couldn't create config dir",
                    info::MessageType::Error,
                );
                exit(1);
            }
        }
        match File::create(params.config_path.as_str()) {
            Ok(_) => (),
            Err(_) => {
                info::print_info(
                    &term,
                    "failed creating config file",
                    info::MessageType::Error,
                );
                exit(1);
            }
        }
    }
    if !Path::new(format!("{}/dicts", params.config_path).as_str()).exists() {
        match create_dir_all(format!("{}/dicts", params.config_dir).as_str()) {
            Ok(_) => (),
            Err(_) => {
                info::print_info(&term, "couldn't create dicts dir", info::MessageType::Error);
                exit(1);
            }
        }
    }
    let conf: Config = match load_config(params.config_path.as_str()) {
        Ok(c) => c,
        Err(e) => {
            info::print_info(
                &term,
                format!("error loading config: {}", e),
                info::MessageType::Error,
            );
            exit(1);
        }
    };

    let usedb: bool = match params.usedb {
        Some(n) => n,
        None => conf.database.unwrap_or(false),
    };

    if let Some(n) = params.query {
        match query::query(n, params.config_dir.clone(), &conf, usedb) {
            Ok(n) => {
                term.println(pretty_print::pretty_print(n)).unwrap();
            }
            Err(e) => {
                info::print_info(
                    &term,
                    format!("error querying files: {}", e),
                    info::MessageType::Error,
                );
                exit(1);
            }
        }
    }

    // TODO: move this to src/dict.rs and add database implementation
    if params.dict != String::new() {
        let mut parts = params.dict.as_str().split(';').map(|x| x.to_string());
        let dict_fname: String = parts.next().unwrap();
        let name: String = parts.next().unwrap();
        let mut meanings: String = parts.next().unwrap();
        if let Some(n) = parts.next() {
            meanings.push_str(format!(";{}", n).as_str());
        }
        let vocab: Vocab = match Vocab::from_string(format!("{};{}", name, meanings)) {
            Ok(n) => n,
            Err(e) => {
                info::print_info(
                    &term,
                    format!("error parsing vocabulary: {}", e),
                    info::MessageType::Error,
                );
                exit(1);
            }
        };
        let mut file = match params.usedb {
            Some(true) => conf.dbpath.unwrap_or_else(|| "vocab.db".to_string()),
            Some(false) | None => match conf.database {
                Some(true) => conf.dbpath.unwrap_or_else(|| "vocab.db".to_string()),
                Some(false) | None => conf.dict.unwrap_or_else(|| String::from("dicts")),
            },
        };
        if !file.starts_with('/') {
            file = format!("{}/{}", params.config_dir, file);
        }
        match write_vocab(file.as_str(), dict_fname.as_str(), vocab, &term, usedb) {
            Ok(_) => exit(0),
            Err(e) => {
                info::print_info(
                    &term,
                    format!("problems writing vocab: {}", e),
                    info::MessageType::Error,
                );
                exit(1);
            }
        }
    }

    if let Some(n) = params.pretprin {
        let voc: Vec<Vocab> = match load_vocab(params.config_dir.clone(), n, &conf, usedb) {
            Ok(p) => p,
            Err(e) => {
                info::print_info(
                    &term,
                    format!("error while parsing vocabulary dictionary: {}", e),
                    info::MessageType::Error,
                );
                exit(1);
            }
        };
        term.println(pretty_print::pretty_print(voc)).unwrap();
        exit(0);
    }

    if params.lang == String::new() {
        exit(0);
    }
    let vocab = match load_vocab(params.config_dir.clone(), params.lang.clone(), &conf, usedb) {
        Ok(n) => n,
        Err(e) => {
            info::print_info(
                &term,
                format!("error while parsing vocabulary dictionary: {}", e),
                info::MessageType::Error,
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

use std::env::args;
use std::env::var;

pub struct Params {
    pub lang: String,
    pub config_path: String,
    pub config_dir: String,
    pub quit: bool,
    pub dict: String,
    pub vocab: String,
    pub adds: Option<bool>,
    pub pretprin: Option<String>,
    pub clearlines: Option<bool>,
}

impl Params {
    pub fn new() -> Params {
        let user_confdir: String = match var("XDG_CONFIG_HOME") {
            Ok(n) => n,
            Err(_) => format!("{}/.config", var("HOME").unwrap()),
        };
        Params {
            lang: String::new(),
            config_path: format!("{}/vct/config.toml", user_confdir),
            config_dir: format!("{}/vct", user_confdir),
            quit: false,
            dict: String::new(),
            vocab: String::new(),
            adds: None,
            pretprin: None,
            clearlines: None,
        }
    }
}

pub fn load_params() -> Params {
    let arguments: Vec<String> = args().collect();
    let mut params: Params = Params::new();
    if arguments.len() < 2 {
        eprintln!("{}", HELP_STR);
        params.quit = true;
    }
    for (idx, arg) in arguments.clone().into_iter().enumerate() {
        match arg.as_str() {
            "-h" | "--help" => {
                eprintln!("{}", HELP_STR);
                params.quit = true;
            }
            "-v" | "--version" => {
                eprintln!("{}", VERSION_STR);
                params.quit = true;
            }
            "--config" => {
                if (arguments.len() - 1) > idx {
                    params.config_path = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no config path provided");
                }
            }
            "-l" | "--lang" => {
                if (arguments.len() - 1) > idx {
                    params.lang = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no lang provided")
                }
            }
            "--config-dir" => {
                if (arguments.len() - 1) > idx {
                    params.config_dir = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no config dir provided");
                }
            }
            "-d" | "--dict" => {
                if (arguments.len() - 3) > idx {
                    params.dict = format!(
                        "{};{};{}",
                        arguments[idx + 1usize].clone(),
                        arguments[idx + 2usize].clone(),
                        arguments[idx + 3usize].clone()
                    );
                    if (arguments.len() - 4) > idx {
                        if arguments[idx + 4usize].clone().starts_with('-') {
                            continue;
                        } else {
                            params.dict = format!(
                                "{};{};{};{}",
                                arguments[idx + 1usize].clone(),
                                arguments[idx + 2usize].clone(),
                                arguments[idx + 3usize].clone(),
                                arguments[idx + 4usize].clone()
                            );
                        }
                    }
                } else {
                    eprintln!("vct: no parameters for dict operations provided");
                }
            }
            "-V" | "--vocab" => {
                if (arguments.len() - 1) > idx {
                    params.vocab = match arguments[idx + 1usize].clone().as_str() {
                        "all" => String::from("all"),
                        "one" => String::from("one"),
                        n => {
                            eprintln!("vct: warning: '{}' is not valid as a vocab parameter. Valid are 'one' and 'all'. Using default", n);
                            String::new()
                        }
                    }
                }
            }
            "--noadds" => {
                params.adds = Some(false);
            }
            "--adds" => {
                params.adds = Some(true);
            }
            "-p" | "--pretty" => {
                if (arguments.len() - 1) > idx {
                    params.pretprin = Some(arguments[idx + 1usize].clone());
                } else {
                    eprintln!("vct: warning: no lang provided");
                }
            }
            "--clear" => {
                params.clearlines = Some(true);
            }
            "--noclear" => {
                params.clearlines = Some(false);
            }
            _ => (),
        }
    }
    params
}

const HELP_STR: &str = "
Synopsis:
    vct [-hv] [-l <lang>] [-p <lang>]
Usage:
  vct [OPTIONS]
Options:
  -h,--help: print this help page and exit
  -v,--version: print the version and exit
  --config <config>: set a different config path
  --config-dir <confdir>: set a different config dir
  -l,--lang <lang>: set the lang to choose vocabulary from
  -d,--dict <dict> <name> <meanings> [additionals]: add a new entry to an existing dict (meanings is a comma seperated list and additionals a comma seperated list of `key:value` pairs)
  -V,--vocab <vocab>: sets how many vocabs should be trained (all or one)
  --noadds: disable additionals
  --adds: enable additionals
  -p,--pretty <lang>: pretty print the vocabulary of <lang> and quit
  --clear: enable clearing lines (less unused screen space)
  --noclear: disable clearing lines (more unused screen space)
";
const VERSION_STR: &str = "vct: v1.3.12-nightly
commit: 59a8ce1901f089ebbfcd4514cb5b3b5f1802ed95";

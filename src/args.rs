use std::env::args;
use std::env::var;

pub struct Params {
    pub lang: String,
    pub help: bool,
    pub version: bool,
    pub config_path: String,
    pub config_dir: String,
}

impl Params {
    pub fn new() -> Params {
        let homedir: String = var("HOME").unwrap();
        Params {
            lang: String::new(),
            help: false,
            version: false,
            config_path: format!("{}/.config/vct/config.toml", homedir),
            config_dir: format!("{}/.config/vct", homedir),
        }
    }
}

pub fn load_params() -> Params {
    let arguments: Vec<String> = args().collect();
    let mut params: Params = Params::new();
    for (idx, arg) in arguments.clone().into_iter().enumerate() {
        match arg.as_str() {
            "-h" | "--help" => {
                params.help = true;
            }
            "-v" | "--version" => {
                params.version = true;
            }
            "-c" | "--config" => {
                if arguments.len() > idx {
                    params.config_path = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no config path provided");
                }
            }
            "-l" | "--lang" => {
                if arguments.len() > idx {
                    params.lang = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no lang provided")
                }
            }
            "-d" | "--config-dir" => {
                if arguments.len() > idx {
                    params.config_dir = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no config dir provided");
                }
            }
            _ => (),
        }
    }
    params
}

pub const HELP_STR: &str = "
Usage:
  vct [OPTIONS]
Options:
  -h,--help: print this help page and exit
  -v,--version: print the version and exit
  -c,--config <config>: set a different config path
  -l,--lang: set the lang to choose vocabulary from

";
pub const VERSION_STR: &str = "vct: v0.1.0";

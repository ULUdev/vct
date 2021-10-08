use std::env::args;
use std::env::var;

pub struct Params {
    pub lang: String,
    pub config_path: String,
    pub config_dir: String,
    pub quit: bool,
    pub dict: String,
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
            "-c" | "--config" => {
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
            "-d" | "--config-dir" => {
                if (arguments.len() - 1) > idx {
                    params.config_dir = arguments[idx + 1usize].clone();
                } else {
                    eprintln!("vct: no config dir provided");
                }
            }
            "-D" | "--dict" => {
                if (arguments.len() - 3) > idx {
                    params.dict = format!(
                        "{};{};{}",
                        arguments[idx + 1usize].clone(),
                        arguments[idx + 2usize].clone(),
                        arguments[idx + 3usize].clone()
                    );
                } else {
                    eprintln!("vct: no parameters for dict operations provided");
                }
            }
            _ => (),
        }
    }
    params
}

const HELP_STR: &str = "
Usage:
  vct [OPTIONS]
Options:
  -h,--help: print this help page and exit
  -v,--version: print the version and exit
  -c,--config <config>: set a different config path
  -d,--config-dir <confdir>: set a different config dir
  -l,--lang: set the lang to choose vocabulary from
  -D,--dict <dict> <name> <meanings>: add a new entry to an existing dict (meanings is a comma seperated list)
";
const VERSION_STR: &str = "vct: v1.0.0";

use clap::{builder::PossibleValuesParser, Arg, ArgAction, ArgMatches, Command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub auth_url: String,
    pub token_url: String,
    pub instance_url: String,
}

pub fn get_config_path() -> String {
    let home_path = dirs::home_dir().expect("Error: Home directory not found");

    format!("{}/.ssr/ssr.toml", home_path.to_str().unwrap())
}

pub fn get_args() -> ArgMatches {
    Command::new("SSR")
        .arg(
            Arg::new("debug-level")
                .short('d')
                .long("debug-level")
                .action(ArgAction::Set)
                .default_value("info")
                .required(false)
                .help("Set debug level")
                .ignore_case(false)
                .value_parser(PossibleValuesParser::new([
                    "trace", "debug", "info", "warn", "error",
                ])),
        )
        .arg(
            Arg::new("sensor")
                .short('s')
                .long("sensor")
                .action(ArgAction::Set)
                .required(true)
                .help("Set sensor name (Case Sensitive)")
                .ignore_case(true),
        )
        .get_matches()
}

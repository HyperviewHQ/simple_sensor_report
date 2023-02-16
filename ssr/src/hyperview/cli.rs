use clap::{value_parser, Parser};
use log::LevelFilter;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SsrArgs {
    #[arg(short, long, help = "Debug level", default_value = "info", value_parser(["trace", "debug", "info", "warn", "error"]))]
    pub debug_level: String,

    #[arg(
        short = 't',
        long,
        help = "Asset type. e.g. Rack",
        value_parser([
            "BladeEnclosure",
            "BladeNetwork",
            "BladeServer",
            "BladeStorage",
            "Busway",
            "Camera",
            "Chiller",
            "Crac",
            "Crah",
            "Environmental",
            "FireControlPanel",
            "Generator",
            "InRowCooling",
            "KvmSwitch",
            "Location",
            "Monitor",
            "NetworkDevice",
            "NetworkStorage",
            "NodeServer",
            "PatchPanel",
            "PduAndRpp",
            "PowerMeter",
            "Rack",
            "RackPdu",
            "Server",
            "SmallUps",
            "TransferSwitch",
            "Ups",
            "VirtualServer",
        ])
    )]
    pub asset_type: String,

    #[arg(short, long, help = "Sensor name. E.g. averageKwhByHour")]
    pub sensor: String,

    #[arg(
        short,
        long,
        help = "Optional custom property name. E.g. \"Business Unit\""
    )]
    pub custom_property: Option<String>,

    #[arg(short, long, help = "Year value for readings (2020 -> 2029). E.g. 2023", value_parser(value_parser!(i32).range(2020..2030)))]
    pub year: i32,

    #[arg(short, long, help = "Month value for readings (1 -> 12). E.g. 1", value_parser(value_parser!(u32).range(1..13)))]
    pub month: u32,

    #[arg(short, long, help = "Offset number (0 -> 99999). e.g. 100", default_value = "0", value_parser(value_parser!(u32).range(0..100000)))]
    pub offset: u32,

    #[arg(short, long, help = "Record limit (1 -> 250). e.g. 100", default_value = "50", value_parser(value_parser!(u32).range(1..251)))]
    pub limit: u32,
}

pub fn get_debug_filter(debug_level: &String) -> LevelFilter {
    if debug_level == "error" {
        LevelFilter::Error
    } else if debug_level == "warn" {
        LevelFilter::Warn
    } else if debug_level == "debug" {
        LevelFilter::Debug
    } else if debug_level == "trace" {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    }
}

use anyhow::Result;
use clap::Parser;
use hyperview::cli::AppConfig;
use log::{info, trace, LevelFilter};

use crate::hyperview::{api::get_asset_list, cli::get_config_path};

mod hyperview;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Debug level", default_value = "info", value_parser(["trace", "debug", "info", "warn", "error"]))]
    debug_level: String,

    #[arg(short, long, help = "Sensor name")]
    sensor: String,

    #[arg(short, long, help = "Optional custom property name")]
    custom_property: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let debug_level = args.debug_level.clone();

    let level_filter = get_debug_filter(&debug_level);
    env_logger::builder().filter(None, level_filter).init();

    info!("Starting ssr");
    info!(
        "Startup options:\n - debug level: {}\n - sensor: {} \n - custom property: {:?}",
        args.debug_level, args.sensor, args.custom_property
    );

    let config: AppConfig = confy::load_path(get_config_path())?;
    info!("Connecting to: {}", config.instance_url);

    let query = vec![("assetType", "rack"), ("(limit)", "1000")];

    let asset_list = get_asset_list(
        &config,
        query,
        "Business Unit".to_string(),
        "averageKwhByHour".to_string(),
        2023,
        2,
    );

    trace!("{:#?}", asset_list);

    Ok(())
}

fn get_debug_filter(debug_level: &String) -> LevelFilter {
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

use anyhow::Result;
use chrono::NaiveDate;
use hyperview::cli::AppConfig;
use log::{info, trace, LevelFilter};

use crate::hyperview::{
    api::get_asset_list,
    cli::{get_args, get_config_path},
};

mod hyperview;

fn main() -> Result<()> {
    let args = get_args();

    if let Some(debug_level) = args.get_one::<String>("debug-level") {
        let level_filter = get_debug_filter(debug_level);

        env_logger::builder().filter(None, level_filter).init();
    } else {
        env_logger::init();
    };

    info!("Starting ssr");

    let config: AppConfig = confy::load_path(get_config_path())?;
    trace!("Config: \n{:#?}", config);

    let query = "?assetType=rack&(limit)=1000".to_string();
    let asset_list = get_asset_list(
        &config,
        query,
        "Business Unit".to_string(),
        "averageKwhByHour".to_string(),
        2023,
        2,
    );

    trace!("{:#?}", asset_list);

    let d = "2023-02-01T00:00:00.000";
    let pd = NaiveDate::parse_from_str(d, "%Y-%m-%dT%H:%M:%S%.f")?;
    info!("DEBUG parsed time: {}", pd.to_string());

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

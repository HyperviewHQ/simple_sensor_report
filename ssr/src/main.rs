use anyhow::Result;
use chrono::NaiveDate;
use hyperview::cli::AppConfig;
use log::{info, trace};

use crate::hyperview::{api::get_asset_list, cli::get_config_path};

mod hyperview;

fn main() -> Result<()> {
    env_logger::init();
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

    println!("{:#?}", asset_list);

    let d = "2023-02-01T00:00:00.000";
    let pd = NaiveDate::parse_from_str(d, "%Y-%m-%dT%H:%M:%S%.f")?;
    info!("DEBUG parsed time: {}", pd.to_string());

    Ok(())
}

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
    );

    println!("{:#?}", asset_list);

    let year = 2023;
    let month = 2;

    let num_of_days = NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .expect("Invalid date")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date"))
    .num_days();

    println!("Number of days: {}", num_of_days);

    Ok(())
}

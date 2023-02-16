use anyhow::Result;
use clap::Parser;
use hyperview::cli::AppConfig;
use log::info;

use crate::hyperview::{
    api::get_asset_list,
    cli::{get_config_path, get_debug_filter, SsrArgs},
};

mod hyperview;

fn main() -> Result<()> {
    let args = SsrArgs::parse();

    let debug_level = args.debug_level;
    let sensor = args.sensor;
    let custom_property = args.custom_property;
    let year = args.year;
    let month = args.month;
    let asset_type = args.asset_type;
    let offset = args.offset.to_string();
    let limit = args.limit.to_string();

    let level_filter = get_debug_filter(&debug_level);
    env_logger::builder().filter(None, level_filter).init();

    info!("Starting ssr");
    info!(
        "\nStartup options:\n| asset type: {} | debug level: {} | sensor: {} | custom property: {:?} | offset: {} | limit: {} |\n",
        asset_type, debug_level, sensor, custom_property, offset, limit
    );

    let config: AppConfig = confy::load_path(get_config_path())?;
    info!("Connecting to: {}", config.instance_url);

    let query = vec![
        ("assetType", asset_type.as_str()),
        ("(after)", &offset),
        ("(limit)", &limit),
        ("(sort)", "+Id"),
    ];

    let asset_list = get_asset_list(&config, query, custom_property, sensor, year, month)?;

    for asset in asset_list {
        let cp = if let Some(cp) = asset.custom_property {
            cp
        } else {
            "N/A".to_string()
        };

        let sn = asset.sensor_name.unwrap();

        let sid = asset.sensor_id.unwrap();

        let su = if let Some(su) = asset.sensor_unit {
            su
        } else {
            "N/A".to_string()
        };

        for reading in asset.sensor_data_points {
            println!(
                "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                asset.name,
                asset.id,
                cp,
                sn,
                sid,
                su,
                reading.r,
                reading.avg,
                reading.max,
                reading.min,
                reading.lst
            );
        }
    }

    Ok(())
}

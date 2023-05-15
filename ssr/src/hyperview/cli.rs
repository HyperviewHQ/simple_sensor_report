use anyhow::Result;
use clap::{value_parser, Parser};
use csv::Writer;
use log::{LevelFilter, debug};
use serde::{Deserialize, Serialize};

use super::api::BasicAsset;

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

    format!("{}/.hyperview/hyperview.toml", home_path.to_str().unwrap())
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

    #[arg(
        short = 'f',
        long,
        help = "Name of output csv file. e.g. sensor_data_2023_02.csv"
    )]
    pub output_file: String,
}

#[derive(Debug, Serialize)]
struct SensorReadingRow {
    asset_name: String,
    asset_id: String,
    custom_property: String,
    sensor_name: String,
    sensor_id: String,
    sensor_unit: String,
    timestamp: String,
    avg: f64,
    max: f64,
    min: f64,
    lst: f64,
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

pub fn write_output(filename: String, asset_list: Vec<BasicAsset>) -> Result<()> {
    let mut writer = Writer::from_path(filename)?;

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
            debug!("{:?}", reading);

            writer.serialize(SensorReadingRow {
                asset_name: asset.name.clone(),
                asset_id: asset.id.clone(),
                custom_property: cp.clone(),
                sensor_name: sn.clone(),
                sensor_id: sid.clone(),
                sensor_unit: su.clone(),
                timestamp: reading.r,
                avg: reading.avg,
                max: reading.max,
                min: reading.min,
                lst: reading.lst,
            })?;
        }
    }

    Ok(())
}

use std::path::Path;

use clap::Parser;
use hyperview::{auth::get_auth_header, cli::AppConfig};
use log::{debug, error, info};
use reqwest::Client;
use serde_json::{Map, Value};

use crate::hyperview::{
    api_functions::get_asset_list,
    cli::{get_config_path, get_debug_filter, write_output, SsrArgs},
    ssr_errors::SsrError,
};

mod hyperview;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = SsrArgs::parse();

    if Path::new(&args.output_file).exists() {
        error!("Specified output file already exists. exiting ...");
        return Err(SsrError::OutputFileExists.into());
    }

    let level_filter = get_debug_filter(&args.debug_level);
    env_logger::builder().filter(None, level_filter).init();

    info!(
        "\nStartup options:\n| asset type: {} | debug level: {} | sensor: {} | custom property: {:?} | offset: {} | limit: {} |\n",
        args.asset_type, args.debug_level, args.sensor, args.custom_property, args.offset, args.limit
    );

    let config: AppConfig = confy::load_path(get_config_path())?;
    let auth_header = get_auth_header(&config).await?;
    let http_client = Client::new();

    debug!("Connecting to: {}", config.instance_url);

    let mut query_params = Map::new();
    query_params.insert("assetType".to_string(), Value::String(args.asset_type));
    query_params.insert("(after)".to_string(), Value::Number(args.offset.into()));
    query_params.insert("(limit)".to_string(), Value::Number(args.limit.into()));
    query_params.insert("(sort)".to_string(), Value::String("+Id".to_string()));

    let asset_list = get_asset_list(
        &config,
        query_params,
        args.custom_property,
        args.sensor,
        args.year,
        args.month,
        &http_client,
        &auth_header,
    )
    .await?;

    info!("Writing data to output file: {}", args.output_file);
    write_output(args.output_file, asset_list)?;

    Ok(())
}

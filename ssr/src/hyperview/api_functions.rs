use chrono::NaiveDate;
use core::time;
use log::{debug, info, trace};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde_json::{Map, Value};
use std::thread;

use super::{
    api_constants::ASSET_API_PREFIX, api_constants::ASSET_CUSTOM_PROPERTIES,
    api_constants::ASSET_NUMERIC_SENSOR_DAILY_SUMMARY, api_constants::ASSET_SENSORS,
    api_data::BasicAsset, api_data::CustomProperty, api_data::NumericSensorResponse,
    cli::AppConfig, ssr_errors::SsrError,
};

pub async fn get_asset_list(
    config: &AppConfig,
    query_params: Map<String, Value>,
    custom_property: Option<String>,
    sensor_name: String,
    year: i32,
    month: u32,
    http_client: &Client,
    auth_header: &String,
) -> anyhow::Result<Vec<BasicAsset>> {
    let mut basic_assets: Vec<BasicAsset> = Vec::new();

    debug!("Processing assets");

    // format target
    let target_url = format!("{}{}", config.instance_url, ASSET_API_PREFIX);
    debug!("Request URL: {:?}", target_url);

    // Get response
    let resp = http_client
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .query(&query_params)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let mut total = 0;
    let mut limit = 0;

    if let Some(metadata) = &resp.get("_metadata") {
        total = metadata["total"].as_u64().unwrap();
        limit = metadata["limit"].as_u64().unwrap();
        info!("\nMeta Data:\n| total: {} | limit: {} |\n", total, limit);
    }

    let end = if limit < total {
        limit as usize
    } else {
        total as usize
    };
    debug!("End: {}", end);

    if let Some(assets) = &resp.get("data") {
        for i in 0..end {
            let id = assets[i]["id"].as_str().unwrap().to_string();
            let name = assets[i]["name"].as_str().unwrap().to_string();
            debug!("id: {}, name: {}", id, name);

            basic_assets.push(BasicAsset {
                id,
                name,
                ..Default::default()
            });
        }
    };

    let throttle = time::Duration::from_millis(100);
    thread::sleep(throttle);

    if let Some(cp) = custom_property {
        info!("Processing custom properties");
        get_asset_custom_properties(config, &mut basic_assets, cp, http_client, auth_header)
            .await?;
    }

    thread::sleep(throttle);
    info!("Processing sensors");
    get_asset_sensors(
        config,
        &mut basic_assets,
        sensor_name,
        http_client,
        auth_header,
    )
    .await?;

    thread::sleep(throttle);
    info!("Processing sensor data");
    let sensor_data = get_numeric_sensor_monthly_summary(
        config,
        &mut basic_assets,
        year,
        month,
        http_client,
        auth_header,
    )
    .await?;
    map_sensor_data_to_asset(&mut basic_assets, &sensor_data);

    Ok(basic_assets)
}

async fn get_asset_custom_properties(
    config: &AppConfig,
    asset_list: &mut Vec<BasicAsset>,
    custom_property: String,
    http_client: &Client,
    auth_header: &String,
) -> anyhow::Result<()> {
    for asset in asset_list {
        // format target
        let target_url = format!(
            "{}{}/{}",
            config.instance_url, ASSET_CUSTOM_PROPERTIES, asset.id
        );
        debug!("Request URL: {:?}", target_url);

        // Get response
        let resp = http_client
            .get(target_url)
            .header(AUTHORIZATION, auth_header)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?
            .json::<Vec<CustomProperty>>()
            .await?;

        debug!("Reading custom properties for asset: {}", asset.id);

        for prop in resp.iter() {
            if prop.name.trim().to_lowercase() == custom_property.trim().to_lowercase() {
                asset.custom_property = Some(prop.value.clone());
            }
        }

        let throttle = time::Duration::from_millis(10);
        thread::sleep(throttle);
    }

    Ok(())
}

async fn get_asset_sensors(
    config: &AppConfig,
    asset_list: &mut Vec<BasicAsset>,
    sensor_name: String,
    http_client: &Client,
    auth_header: &String,
) -> anyhow::Result<()> {
    for asset in asset_list {
        // format target
        let target_url = format!("{}{}/{}", config.instance_url, ASSET_SENSORS, asset.id);
        debug!("Request URL: {:?}", target_url);

        // Get response
        let resp = http_client
            .get(target_url)
            .header(AUTHORIZATION, auth_header)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?
            .json::<Vec<Value>>()
            .await?;

        for sensor in resp.iter() {
            let is_numeric = sensor["isNumeric"].as_bool().unwrap();
            let name = sensor["name"].as_str().unwrap().to_string();
            let id = sensor["id"].as_str().unwrap().to_string();

            let unit = if let Some(u) = sensor["unitString"].as_str() {
                u.to_string()
            } else {
                "N/A".to_string()
            };

            if name == sensor_name {
                if !is_numeric {
                    return Err(SsrError::NonNumericSensorUsed.into());
                }
                asset.sensor_name = Some(name);
                asset.sensor_id = Some(id.clone());
                asset.sensor_unit = Some(unit);

                debug!("Hit on sensor: {} for asset: {}", id, asset.id);
            }
        }
    }

    Ok(())
}

async fn get_numeric_sensor_monthly_summary(
    config: &AppConfig,
    asset_list: &mut Vec<BasicAsset>,
    year: i32,
    month: u32,
    http_client: &Client,
    auth_header: &String,
) -> anyhow::Result<Vec<NumericSensorResponse>> {
    // format target
    let target_url = format!(
        "{}{}",
        config.instance_url, ASSET_NUMERIC_SENSOR_DAILY_SUMMARY
    );
    debug!("Request URL: {:?}", target_url);

    let mut sensor_data: Vec<NumericSensorResponse> = Vec::new();

    let chunk = 100;
    let mut done = false;
    let mut start = 0;
    let mut end = chunk;

    loop {
        if end > asset_list.len() {
            end = asset_list.len();
            done = true;
        }

        debug!("Fetching sensor chunk: {} -> {}", start, end);

        let mut query: Vec<(&str, &str)> = Vec::new();

        for i in asset_list.iter().take(end).skip(start) {
            if let Some(sensor_id) = &i.sensor_id {
                query.push(("sensorIds", sensor_id));
            }
        }

        let period_end = get_next_date(year, month)?;
        let start_time = format!("{}-{}-1T00:00:00.000", year, month);
        let end_time = format!("{}T00:00:00.000", period_end.format("%Y-%m-%d"));

        query.push(("startTime", &start_time));
        query.push(("endTime", &end_time));
        trace!("{:#?}", query);

        // Get response
        let mut resp = http_client
            .get(&target_url)
            .header(AUTHORIZATION, auth_header)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .query(&query)
            .send()
            .await?
            .json::<Vec<NumericSensorResponse>>()
            .await?;

        sensor_data.append(&mut resp);

        if done {
            break;
        }

        start += chunk;
        end += chunk;
    }

    Ok(sensor_data)
}

fn map_sensor_data_to_asset(
    asset_list: &mut Vec<BasicAsset>,
    sensor_data: &[NumericSensorResponse],
) {
    for asset in asset_list {
        if let Some(sensor_id) = &asset.sensor_id {
            let numeric_sensor_response: Vec<_> = sensor_data
                .iter()
                .filter(|s| s.sensor_id == *sensor_id)
                .collect();

            if let Some(sensor_data) = numeric_sensor_response.first() {
                asset.sensor_data_points = sensor_data.sensor_data_points.clone();
            }
        }
    }
}

fn get_next_date(year: i32, month: u32) -> anyhow::Result<NaiveDate> {
    if let Some(_start) = NaiveDate::from_ymd_opt(year, month, 1) {
        let e_year = match month {
            12 => year + 1,
            _ => year,
        };

        let e_month = match month {
            12 => 1,
            _ => month + 1,
        };

        Ok(NaiveDate::from_ymd_opt(e_year, e_month, 1).unwrap())
    } else {
        Err(SsrError::YearMonthConversion.into())
    }
}

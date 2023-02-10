use super::{auth::get_auth_header, cli::AppConfig};
use anyhow::Result;
use chrono::NaiveDate;
use log::{debug, info, trace};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

const ASSET_API_PREFIX: &str = "/api/asset/assets";
const ASSET_CUSTOM_PROPERTIES: &str = "/api/asset/customAssetProperties";
const ASSET_SENSORS: &str = "/api/asset/sensors";
const ASSET_NUMERIC_SENSOR_DAILY_SUMMARY: &str = "/api/asset/sensorsDailySummaries/numeric";

#[derive(Debug, Error)]
enum SsrError {
    #[error("Could not convert provided year and month")]
    YearMonthConversionError,
    #[error("Invalid sensor type. Only numeric sensors are supported")]
    NonNumericSensorUsedError,
}

#[derive(Debug, Default)]
pub struct BasicAsset {
    pub id: String,
    pub name: String,
    pub custom_property: Option<String>,
    pub sensor_name: Option<String>,
    pub sensor_id: Option<String>,
    pub sensor_unit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CustomProperty {
    id: String,
    #[serde(alias = "customAssetPropertyKeyId")]
    custom_asset_property_key_id: String,
    #[serde(alias = "customAssetPropertyGroupId")]
    custom_asset_property_group_id: String,
    value: String,
    #[serde(alias = "dataType")]
    data_type: String,
    name: String,
    #[serde(alias = "groupName")]
    group_name: String,
    #[serde(alias = "dataSource")]
    data_source: String,
    #[serde(alias = "updatedDateTime")]
    updated_date_time: String,
    unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NumericSensorResponse {
    #[serde(alias = "sensorId")]
    sensor_id: String,
    #[serde(alias = "sensorTypeDescription")]
    sensor_type_description: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    name: String,
    #[serde(alias = "sensorDataPoints")]
    sensor_data_points: Vec<NumericSensorDailySummaryDataPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NumericSensorDailySummaryDataPoint {
    r: String,
    avg: f64,
    max: f64,
    min: f64,
    lst: f64,
}

pub fn get_asset_list(
    config: &AppConfig,
    query: String,
    custom_property: String,
    sensor_name: String,
    year: i32,
    month: u32,
) -> Result<Vec<BasicAsset>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;
    debug!("Auth header: {}", auth_header);

    let mut basic_assets: Vec<BasicAsset> = Vec::new();

    // format target
    let target_url = format!("{}{}{}", config.instance_url, ASSET_API_PREFIX, query);
    debug!("Target URL: {:?}", target_url);

    info!("Getting asset list");
    // Start http client
    let req = reqwest::blocking::Client::new();

    // Get response
    let resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()?
        .json::<Value>()?;

    let mut total = 0;
    let mut limit = 0;

    if let Some(metadata) = &resp.get("_metadata") {
        total = metadata["total"].as_u64().unwrap();
        limit = metadata["limit"].as_u64().unwrap();
        debug!("Total records found: {}, quey limit: {}", total, limit);
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

    info!("Getting custom property values");
    get_asset_custom_properties(config, &mut basic_assets, custom_property)?;

    info!("Getting sensor ids");
    get_asset_sensors(config, &mut basic_assets, sensor_name)?;

    info!("Getting sensor data for month");
    get_numeric_sensor_monthly_summary(config, &mut basic_assets, year, month)?;

    Ok(basic_assets)
}

fn get_asset_custom_properties(
    config: &AppConfig,
    asset_list: &mut Vec<BasicAsset>,
    custom_property: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;
    debug!("Auth header: {}", auth_header);

    for asset in asset_list {
        // format target
        let target_url = format!(
            "{}{}/{}",
            config.instance_url, ASSET_CUSTOM_PROPERTIES, asset.id
        );
        debug!("Target URL: {:?}", target_url);

        // Start http client
        let req = reqwest::blocking::Client::new();

        // Get response
        let resp = req
            .get(target_url)
            .header(AUTHORIZATION, auth_header.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()?
            .json::<Vec<CustomProperty>>()?;

        for prop in resp.iter() {
            trace!("{:#?}", prop);
            if prop.name.trim().to_lowercase() == custom_property.trim().to_lowercase() {
                asset.custom_property = Some(prop.value.clone());
            }
        }
    }

    Ok(())
}

fn get_asset_sensors(
    config: &AppConfig,
    asset_list: &mut Vec<BasicAsset>,
    sensor_name: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;
    debug!("Auth header: {}", auth_header);

    for asset in asset_list {
        // format target
        let target_url = format!("{}{}/{}", config.instance_url, ASSET_SENSORS, asset.id);
        debug!("Target URL: {:?}", target_url);

        // Start http client
        let req = reqwest::blocking::Client::new();

        // Get response
        let resp = req
            .get(target_url)
            .header(AUTHORIZATION, auth_header.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()?
            .json::<Vec<Value>>()?;

        for sensor in resp.iter() {
            let is_numeric = sensor["isNumeric"].as_bool().unwrap();
            if !is_numeric {
                return Err(SsrError::NonNumericSensorUsedError.into());
            }

            let name = sensor["name"].as_str().unwrap().to_string();
            let id = sensor["id"].as_str().unwrap().to_string();
            let unit = if let Some(u) = sensor["unitString"].as_str() {
                u.to_string()
            } else {
                "N/A".to_string()
            };

            trace!("{:#?}", sensor);
            if name == sensor_name {
                asset.sensor_name = Some(name);
                asset.sensor_id = Some(id);
                asset.sensor_unit = Some(unit);
            }
        }
    }

    Ok(())
}

fn get_numeric_sensor_monthly_summary(
    config: &AppConfig,
    asset_list: &mut Vec<BasicAsset>,
    year: i32,
    month: u32,
) -> Result<()> {
    let end = get_next_date(year, month)?;

    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;
    debug!("Auth header: {}", auth_header);

    // format target
    let target_url = format!(
        "{}{}",
        config.instance_url, ASSET_NUMERIC_SENSOR_DAILY_SUMMARY
    );
    debug!("Target URL: {:?}", target_url);

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut query: Vec<(String, String)> = Vec::new();

    for asset in asset_list {
        if let Some(sensor_id) = &asset.sensor_id {
            query.push(("sensorIds".to_string(), sensor_id.to_string()));
        }
    }

    let start_time = format!("{}-{}-1T00:00:00.000", year, month);
    let end_time = format!("{}T00:00:00.000", end.format("%Y-%m-%d").to_string());

    query.push(("startTime".to_string(), start_time));
    query.push(("endTime".to_string(), end_time));
    trace!("{:#?}", query);

    // Get response
    let resp = req
        .get(target_url.clone())
        .header(AUTHORIZATION, auth_header.clone())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .query(&query)
        .send()?
        .json::<Vec<NumericSensorResponse>>()?;

    trace!("{:#?}", resp);

    Ok(())
}

fn get_number_of_days_in_month(year: i32, month: u32) -> Result<i64> {
    if let Some(start) = NaiveDate::from_ymd_opt(year, month, 1) {
        let end = get_next_date(year, month)?;
        Ok(end.signed_duration_since(start).num_days())
    } else {
        Err(SsrError::YearMonthConversionError.into())
    }
}

fn get_next_date(year: i32, month: u32) -> Result<NaiveDate> {
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
        Err(SsrError::YearMonthConversionError.into())
    }
}
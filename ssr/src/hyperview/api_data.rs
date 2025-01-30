use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};

#[derive(Debug, Default)]
pub struct BasicAsset {
    pub id: String,
    pub name: String,
    pub custom_property: Option<String>,
    pub sensor_name: Option<String>,
    pub sensor_id: Option<String>,
    pub sensor_unit: Option<String>,
    pub sensor_data_points: Vec<NumericSensorDailySummaryDataPoint>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomProperty {
    pub id: String,
    #[serde(alias = "customAssetPropertyKeyId")]
    pub custom_asset_property_key_id: String,
    #[serde(alias = "customAssetPropertyGroupId")]
    pub custom_asset_property_group_id: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub value: String,
    #[serde(alias = "dataType")]
    pub data_type: String,
    pub name: String,
    #[serde(alias = "groupName")]
    pub group_name: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(alias = "dataSource")]
    pub data_source: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(alias = "updatedDateTime")]
    pub updated_date_time: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NumericSensorResponse {
    #[serde(alias = "sensorId")]
    pub sensor_id: String,
    #[serde(alias = "sensorTypeDescription")]
    pub sensor_type_description: String,
    #[serde(alias = "sensorTypeId")]
    pub sensor_type_id: String,
    pub name: String,
    #[serde(alias = "sensorDataPoints")]
    pub sensor_data_points: Vec<NumericSensorDailySummaryDataPoint>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct NumericSensorDailySummaryDataPoint {
    pub r: String,
    pub avg: f64,
    pub max: f64,
    pub min: f64,
    pub lst: f64,
}

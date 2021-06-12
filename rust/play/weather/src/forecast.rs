use chrono::{offset::Local, DateTime};
use serde::Deserialize;
use serde_json::Value;

use crate::types::{Geometry, Quantity, Url};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Forecast {
    #[serde(rename = "@context")]
    context: Value,
    #[serde(rename = "type")]
    type_of: String,
    properties: ForecastProperties,
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForecastProperties {
    updated: DateTime<Local>,
    units: String,
    forecast_generator: String,
    generated_at: DateTime<Local>,
    update_time: DateTime<Local>,
    valid_times: String, // TODO: some kind of datetime range
    elevation: Quantity,
    periods: Vec<Period>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Period {
    number: u32,
    name: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    is_day_time: bool,
    temperature: u32,
    temperature_unit: String,
    temperature_trend: Value, // TODO: shows `null` right now
    wind_speed: String,
    wind_direction: String,
    icon: Url,
    short_forecast: String,
    detailed_forecast: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GridpointData {
}

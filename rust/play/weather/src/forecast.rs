use chrono::{offset::FixedOffset, DateTime};
use geojson::Geometry;
use serde::Deserialize;
use serde_json::Value;

use crate::types::{Angle, TimeInterval, Percent, Quantity, Url};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    #[serde(rename = "@context")]
    pub context: Value,
    #[serde(rename = "type")]
    pub type_of: String,
    pub properties: ForecastProperties,
    pub geometry: Geometry,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastProperties {
    pub updated: DateTime<FixedOffset>,
    pub units: String,
    pub forecast_generator: String,
    pub generated_at: DateTime<FixedOffset>,
    pub update_time: DateTime<FixedOffset>,
    pub valid_times: TimeInterval,
    pub elevation: Quantity,
    pub periods: Vec<Period>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub number: u32,
    pub name: String,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub is_daytime: bool,
    pub temperature: u32,
    pub temperature_unit: String,
    pub temperature_trend: Option<Value>,
    pub wind_speed: String,
    pub wind_direction: String,
    pub icon: Url,
    pub short_forecast: String,
    pub detailed_forecast: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridpointData {
    #[serde(rename = "@context")]
    pub context: Value,
    pub id: Url,
    #[serde(rename = "type")]
    pub type_of: String,
    pub geometry: Geometry,
    pub properties: GridpointDataProperties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridpointDataProperties {
    #[serde(rename = "@id")]
    pub id: Url,
    #[serde(rename = "@type")]
    pub type_of: String,
    pub update_time: DateTime<FixedOffset>,
    pub valid_times: TimeInterval,
    pub elevation: Quantity,
    pub forecast_office: Url,
    pub grid_id: String,
    pub grid_x: String,
    pub grid_y: String,
    pub temperature: TimeSeries<f64>,
    pub dewpoint: TimeSeries<f64>,
    pub max_temperature: TimeSeries<f64>,
    pub min_temperature: TimeSeries<f64>,
    pub relative_humidity: TimeSeries<Percent>,
    pub apparent_temperature: TimeSeries<f64>,
    pub heat_index: TimeSeries<f64>,
    pub wind_chill: TimeSeries<f64>,
    pub sky_cover: TimeSeries<Percent>,
    pub wind_direction: TimeSeries<Angle>,
    pub wind_speed: TimeSeries<f64>,
    pub wind_gust: TimeSeries<f64>,
    pub weather: TimeSeries<Vec<WeatherDatum>>,
    pub hazards: TimeSeries<Value>,
    pub probability_of_precipitation: TimeSeries<Percent>,
    pub quantitative_precipitation: TimeSeries<f64>,
    pub ice_accumulation: TimeSeries<Value>, // TODO: all `0`.. maybe u32
    pub snowfall_amount: TimeSeries<Value>, // TODO: all `0`.. maybe u32
    pub snow_level: TimeSeries<f64>,
    pub ceiling_height: TimeSeries<Value>,
    pub visibility: TimeSeries<Value>,
    pub transport_wind_speed: TimeSeries<f64>,
    pub transport_wind_direction: TimeSeries<Angle>,
    pub mixing_height: TimeSeries<f64>,
    pub haines_index: TimeSeries<Value>, // TODO: maybe u8
    pub lightning_activity_level: TimeSeries<Value>, // TODO: maybe u8
    pub twenty_foot_wind_speed: TimeSeries<f64>,
    pub twenty_foot_wind_direction: TimeSeries<Angle>,
    pub wave_height: TimeSeries<Value>,
    pub wave_period: TimeSeries<Value>,
    pub wave_direction: TimeSeries<Value>,
    pub primary_swell_height: TimeSeries<Value>,
    pub primary_swell_direction: TimeSeries<Value>,
    pub secondary_swell_height: TimeSeries<Value>,
    pub secondary_swell_direction: TimeSeries<Value>,
    pub wave_period2: TimeSeries<Value>,
    pub wind_wave_height: TimeSeries<Value>,
    pub dispersion_index: TimeSeries<Value>,
    pub pressure: TimeSeries<Value>,
    pub probability_of_tropical_storm_winds: TimeSeries<Value>,
    pub probability_of_hurricane_winds: TimeSeries<Value>,
    pub potential_of_15mph_winds: TimeSeries<Value>,
    pub potential_of_25mph_winds: TimeSeries<Value>,
    pub potential_of_35mph_winds: TimeSeries<Value>,
    pub potential_of_45mph_winds: TimeSeries<Value>,
    pub potential_of_20mph_wind_gusts: TimeSeries<Value>,
    pub potential_of_30mph_wind_gusts: TimeSeries<Value>,
    pub potential_of_40mph_wind_gusts: TimeSeries<Value>,
    pub potential_of_50mph_wind_gusts: TimeSeries<Value>,
    pub potential_of_60mph_wind_gusts: TimeSeries<Value>,
    pub grassland_fire_danger_index: TimeSeries<Value>,
    pub probability_of_thunder: TimeSeries<Value>,
    pub davis_stability_index: TimeSeries<Value>, // TODO: maybe u8
    pub atmospheric_dispersion_index: TimeSeries<Value>,
    pub low_visibility_occurrence_risk_index: TimeSeries<Value>,
    pub stability: TimeSeries<Value>,
    pub red_flag_threat_index: TimeSeries<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries<T> {
    uom: Option<String>,
    values: Vec<TimeDatum<T>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeDatum<T> {
    valid_time: Option<TimeInterval>,
    value: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherDatum {
    coverage: Option<String>,
    weather: Option<String>,
    intensity: Option<String>,
    visibility: Quantity,
    attributes: Vec<Value>,
}

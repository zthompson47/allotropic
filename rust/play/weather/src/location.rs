use chrono_tz::Tz;
use geojson::Geometry;
use serde::Deserialize;

use crate::types::{Quantity, Url};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub id: String,
    pub geometry: Geometry,
    pub properties: PointProperties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointProperties {
    #[serde(rename = "@id")]
    pub id: Url,
    pub cwa: String,
    pub forecast_office: Url,
    pub grid_id: String,
    pub grid_x: u32,
    pub grid_y: u32,
    pub forecast: Url,
    pub forecast_hourly: Url,
    pub forecast_grid_data: Url,
    pub observation_stations: Url,
    pub relative_location: RelativeLocation,
    pub forecast_zone: Url,
    pub county: Url,
    pub fire_weather_zone: Url,
    pub time_zone: Tz,
    pub radar_station: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeLocation {
    pub geometry: Geometry,
    pub properties: RelativeLocationProperties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeLocationProperties {
    pub city: String,
    pub state: String,
    pub distance: Quantity,
    pub bearing: Quantity,
}

impl Point {
    pub fn city(&self) -> &str {
        &self.properties.relative_location.properties.city
    }

    pub fn state(&self) -> &str {
        &self.properties.relative_location.properties.state
    }
}

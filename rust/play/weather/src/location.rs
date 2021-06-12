//#![allow(dead_code)]
//#![allow(unused_variables)]
use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

use crate::error::Result;

pub type Url = String;
type Position = Vec<f64>;

pub const API: &str = "https://api.weather.gov";
pub const APP: &str = "weather.allotropic.com";
pub const USER: &str = "zach@allotropic.com";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    id: String,
    geometry: Geometry,
    properties: PointProperties,
}

#[derive(Debug)]
pub struct ApiClient {
    cache: HashMap<Url, String>,
    client: Client,
    endpoint: Url,
}

impl ApiClient {
    pub fn new(api: &str, app: &str, user: &str) -> Result<Self> {
        Ok(ApiClient {
            cache: HashMap::new(),
            client: Client::builder()
                .user_agent(format!("({}, {})", app, user))
                .build()?,
            endpoint: api.into(),
        })
    }

    pub async fn get_point(&mut self, coordinates: Position) -> Result<Point> {
        let coords = format!(
            "{},{}",
            round_fmt(coordinates[0], 4),
            round_fmt(coordinates[1], 4)
        );
        let url = format!("{}/points/{}", self.endpoint, coords);
        let json = match self.fetch_cached_json(&url) {
            Some(ref json) => {
                println!("CACHE HIT!!!!!!!!!!");
                *json
            }
            None => {
                let response = self.client.get(&url).send().await?;
                let text = response.text().await?;
                //println!("---------->>>{}<<<<<<<<<<---------", text);
                self.cache.insert(url.clone(), text);
                self.cache.get(&url).unwrap()
            }
        };

        Ok(serde_json::from_str(json)?)
    }

    #[allow(clippy::ptr_arg)]
    fn fetch_cached_json(&self, request: &Url) -> Option<&String> {
        self.cache.get(request)
    }
}

fn round_fmt(f: f64, digits: u32) -> String {
    let pow = 10u32.pow(digits) as f64;
    let f = (f * pow).round() / pow;

    format!("{0:.1$}", f, digits as usize)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    type_of: String, // TODO: should be enum
    coordinates: Position,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PointProperties {
    #[serde(rename = "@id")]
    id: Url,
    cwa: String,
    forecast_office: Url,
    grid_id: String,
    grid_x: u32,
    grid_y: u32,
    forecast: Url,
    forecast_hourly: Url,
    forecast_grid_data: Url,
    observation_stations: Url,
    relative_location: RelativeLocation,
    forecast_zone: Url,
    county: Url,
    fire_weather_zone: Url,
    time_zone: String,
    radar_station: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RelativeLocation {
    geometry: Geometry,
    properties: RelativeLocationProperties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RelativeLocationProperties {
    city: String,
    state: String,
    distance: Quantity,
    bearing: Quantity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    value: f32,
    unit_code: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn get_point_from_cache() {
        // The place
        let latitude: f64 = 42.44645644561855;
        let longitude: f64 = -76.4807390759812;

        // Create cached response
        let coords = format!("{},{}", round_fmt(latitude, 4), round_fmt(longitude, 4));
        let url = format!("{}/points/{}", API, coords);
        let mut client = ApiClient::new(API, APP, USER).unwrap();
        client.cache.insert(url.clone(), JSON.to_string());

        // Run request on cached response
        let point = client.get_point(vec![latitude, longitude]).await.unwrap();
        assert_eq!(point.id, url);
        assert_eq!(
            point.properties.relative_location.properties.city,
            "Forest Home"
        );
        assert_eq!((44, 69), (point.properties.grid_x, point.properties.grid_y));
    }

    #[test]
    fn deserialize_json() {
        #[derive(Debug, Deserialize)]
        struct Test {
            id: String,
        }
        let t: Test = serde_json::from_str(JSON).unwrap();
        assert_ne!(t.id, "");
    }

    #[tokio::test]
    async fn mock_get_point() {
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200)
            .set_body_string(JSON)
            .insert_header("content-type", "application/geo+json");

        Mock::given(method("GET"))
            .and(path("/points/42.4465,-76.4807"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let body = reqwest::get(format!("{}{}", &mock_server.uri(), "/points/"))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let latitude: f64 = 42.44645644561855;
        let longitude: f64 = -76.4807390759812;
        let mut client = ApiClient::new(&mock_server.uri(), APP, USER).unwrap();
        let point = client.get_point(vec![latitude, longitude]).await.unwrap();

        assert_eq!(
            point.properties.relative_location.properties.city,
            "Forest Home"
        );
        assert_eq!((44, 69), (point.properties.grid_x, point.properties.grid_y));
    }

    const JSON: &str = r#"{
    "@context": [
        "https://geojson.org/geojson-ld/geojson-context.jsonld",
        {
            "@version": "1.1",
            "wx": "https://api.weather.gov/ontology#",
            "s": "https://schema.org/",
            "geo": "http://www.opengis.net/ont/geosparql#",
            "unit": "http://codes.wmo.int/common/unit/",
            "@vocab": "https://api.weather.gov/ontology#",
            "geometry": {
                "@id": "s:GeoCoordinates",
                "@type": "geo:wktLiteral"
            },
            "city": "s:addressLocality",
            "state": "s:addressRegion",
            "distance": {
                "@id": "s:Distance",
                "@type": "s:QuantitativeValue"
            },
            "bearing": {
                "@type": "s:QuantitativeValue"
            },
            "value": {
                "@id": "s:value"
            },
            "unitCode": {
                "@id": "s:unitCode",
                "@type": "@id"
            },
            "forecastOffice": {
                "@type": "@id"
            },
            "forecastGridData": {
                "@type": "@id"
            },
            "publicZone": {
                "@type": "@id"
            },
            "county": {
                "@type": "@id"
            }
        }
    ],
    "id": "https://api.weather.gov/points/42.4465,-76.4807",
    "type": "Feature",
    "geometry": {
        "type": "Point",
        "coordinates": [
            -76.480699999999999,
            42.4465
        ]
    },
    "properties": {
        "@id": "https://api.weather.gov/points/42.4465,-76.4807",
        "@type": "wx:Point",
        "cwa": "BGM",
        "forecastOffice": "https://api.weather.gov/offices/BGM",
        "gridId": "BGM",
        "gridX": 44,
        "gridY": 69,
        "forecast": "https://api.weather.gov/gridpoints/BGM/44,69/forecast",
        "forecastHourly": "https://api.weather.gov/gridpoints/BGM/44,69/forecast/hourly",
        "forecastGridData": "https://api.weather.gov/gridpoints/BGM/44,69",
        "observationStations": "https://api.weather.gov/gridpoints/BGM/44,69/stations",
        "relativeLocation": {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [
                    -76.471610999999996,
                    42.453136999999998
                ]
            },
            "properties": {
                "city": "Forest Home",
                "state": "NY",
                "distance": {
                    "value": 1049.1700469784,
                    "unitCode": "unit:m"
                },
                "bearing": {
                    "value": 225,
                    "unitCode": "unit:degrees_true"
                }
            }
        },
        "forecastZone": "https://api.weather.gov/zones/forecast/NYZ025",
        "county": "https://api.weather.gov/zones/county/NYC109",
        "fireWeatherZone": "https://api.weather.gov/zones/fire/NYZ025",
        "timeZone": "America/New_York",
        "radarStation": "KBGM"
    }
}"#;
}

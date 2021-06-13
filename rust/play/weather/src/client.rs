use std::collections::HashMap;

use reqwest::Client;

use crate::{
    error::Result,
    forecast::Forecast,
    location::Point,
    types::{Position, Url},
};

/// Endpoint for the National Weather Service API.
pub const API: &str = "https://api.weather.gov";

#[derive(Debug)]
/// The client for the NWS-API. All weather forecat resources are acquired
/// through this client.
pub struct ApiClient {
    pub cache: HashMap<Url, String>, // TODO: pub only for testing
    client: Client,
    endpoint: Url,
}

impl ApiClient {
    /// Create a new client from an endpoint and user credentials.
    pub fn new(api: &str, app: &str, user: &str) -> Result<Self> {
        Ok(ApiClient {
            cache: HashMap::new(),
            client: Client::builder()
                .user_agent(format!("({}, {})", app, user))
                .build()?,
            endpoint: api.into(),
        })
    }

    /// Translate a latitude and longitude into a gridpoint location in order
    /// to generate weather forecast requests.
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

    /// Fetch a weather forecast from a given url.
    pub async fn get_forecast_from_url(&self, url: String) -> Result<Forecast> {
        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        Ok(serde_json::from_str(&text)?)
    }
}

/// Round a floating point number to a specified number of significant digits.
/// Used to generate latitude and longitude coordinates with four significant
/// digits for NWS-API requests.
pub fn round_fmt(f: f64, digits: u32) -> String {
    let pow = 10u32.pow(digits) as f64;
    let f = (f * pow).round() / pow;

    format!("{0:.1$}", f, digits as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_coords() {
        #![allow(clippy::excessive_precision)]
        let latitude: f64 = 53.473894723894738;
        let longitude: f64 = -39.43784723847389;
        assert_eq!("53.4739".to_string(), round_fmt(latitude, 4));
        assert_eq!("-39.4378".to_string(), round_fmt(longitude, 4));
    }
}

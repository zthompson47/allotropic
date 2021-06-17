//use std::collections::HashMap;

use chrono::{Duration, Utc};
use reqwest::{header::CACHE_CONTROL, Client, IntoUrl};
use rusqlite::ToSql;

use crate::{
    cache::Cache,
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
    cache: Cache,
    client: Client,
    base_url: Url,
}

impl ApiClient {
    /// Create a new client from a url and user credentials.
    pub fn new(base_url: &str, app: &str, user: &str) -> Result<Self> {
        Ok(ApiClient {
            cache: Cache::new()?,
            client: Client::builder()
                .user_agent(format!("({}, {})", app, user))
                .build()?,
            base_url: base_url.into(),
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
        let url = format!("{}/points/{}", self.base_url, coords);
        let json = self.fetch_resource(&url).await?;

        Ok(serde_json::from_str(&json)?)
    }

    async fn fetch_resource<'a, T>(&mut self, url: &'a T) -> Result<String>
    where
        T: AsRef<str> + ToSql,
        &'a T: IntoUrl,
    {
        match self.cache.get(url)? {
            Some(entry) => {
                let max_age = entry.max_age.unwrap_or(0);
                let expires_at = entry.created_at + Duration::seconds(max_age as i64);
                if expires_at <= Utc::now() {
                    // TODO: check last_modified for updated resource
                    Ok(self.get_and_cache(url).await?)
                } else {
                    Ok(entry.content)
                }
            }
            None => Ok(self.get_and_cache(url).await?),
        }
    }

    async fn get_and_cache<'a, T>(&mut self, url: &'a T) -> Result<String>
    where
        T: AsRef<str> + ToSql,
        &'a T: IntoUrl,
    {
        let response = self.client.get(url).send().await?;
        let mut max_age = None;
        if let Some(cache_control) = response.headers().get(CACHE_CONTROL) {
            if let Ok(cache_control) = cache_control.to_str() {
                for mut part in cache_control.split(',') {
                    part = part.trim();
                    if part.starts_with("max-age") { // max-age= and then unwrap splitonece..
                        if let Some((_, v)) = part.split_once('=') {
                            if let Ok(v) = v.parse::<u32>() {
                                max_age = Some(v);
                                break;
                            }
                        }
                    }
                }
            }
        }

        let text = response.text().await?;
        self.cache.insert(url, max_age, Utc::now(), &text)?;
        Ok(text)
    }

    /// Fetch a weather forecast from a given url.
    pub async fn get_forecast_from_url(&mut self, url: String) -> Result<Forecast> {
        let json = self.fetch_resource(&url).await?;
        Ok(serde_json::from_str(&json)?)
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

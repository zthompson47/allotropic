use std::path::PathBuf;

use chrono::{Duration, Utc};
use reqwest::{header::CACHE_CONTROL, Client};

use crate::{
    cache::Cache,
    error::{err, Result},
    forecast::Forecast,
    location::Point,
    types::{Position, Url},
};

/// Base URL for the National Weather Service API.
pub const API: &str = "https://api.weather.gov";

#[derive(Debug)]
/// The client for the NWS-API. All weather forecat resources are acquired
/// through this client.
pub struct ApiClient {
    cache: Cache,
    client: Client,
    base_url: Url,
}

#[derive(Debug, Default)]
pub struct ApiClientBuilder {
    cache_base_dir: Option<PathBuf>,
    api_key: Option<String>,
    api_base_url: Option<Url>,
}

impl ApiClientBuilder {
    pub fn build(self) -> Result<ApiClient> {
        Ok(ApiClient {
            cache: Cache::with_base_dir(self.cache_base_dir)?,
            client: Client::builder()
                .user_agent(self.api_key.unwrap())
                .build()?,
            base_url: self.api_base_url.unwrap(),
        })
    }

    pub fn cache_base_dir(mut self, path: PathBuf) -> Self {
        self.cache_base_dir = Some(path);
        self
    }

    pub fn api_key(mut self, domain: &str, email: &str) -> Self {
        self.api_key = Some(format!("({}, {})", domain, email));
        self
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.api_base_url = Some(url.into());
        self
    }
}

impl ApiClient {
    /// Create a builder to construct a new `ApiClient`.
    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::default()
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

    /// Fetch a weather forecast from a given url, for different time
    /// resolutions.
    pub async fn get_forecast_from_url(&mut self, url: &str) -> Result<Forecast> {
        let json = self.fetch_resource(url).await?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn fetch_resource(&mut self, url: &str) -> Result<String> {
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

    async fn get_and_cache(&mut self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;

        // Bail on error
        let status = response.status();
        if !status.is_success() {
            return err(format!(
                "Unable to connect to {}: {} {}",
                url,
                status.as_str(),
                status.canonical_reason().unwrap_or("")
            )
            .as_str());
        }

        let mut max_age = None;
        if let Some(cache_control) = response.headers().get(CACHE_CONTROL) {
            if let Ok(cache_control) = cache_control.to_str() {
                for mut part in cache_control.split(',') {
                    part = part.trim();
                    if part.starts_with("max-age") {
                        if let Some((_, mut v)) = part.split_once('=') {
                            v = v.trim();
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

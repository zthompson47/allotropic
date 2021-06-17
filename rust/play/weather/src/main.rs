use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use colorgrad::{Color, CustomGradient};
use crossterm::style::{self, Stylize};
use sunrise::sunrise_sunset;

use weather::client::ApiClient;

const API: &str = "https://api.weather.gov";
const APP: &str = "weather.allotropic.com";
const USR: &str = "zach@allotropic.com";
const LAT: f64 = 42.440932990492946;
const LON: f64 = -76.52462385924595;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut client = ApiClient::new(API, APP, USR).unwrap();

    // Find the weather station gridpoint for the location
    let point = client.get_point(vec![LAT, LON]).await.unwrap();

    let forecast = client
        .get_forecast_from_url(point.properties.forecast_hourly)
        .await
        .unwrap();

    let max_wind_column_len = forecast
        .properties
        .periods
        .iter()
        .map(|x| x.wind_speed.len())
        .max()
        .unwrap();

    let _time_grad = CustomGradient::new()
        .colors(&[
            Color::from_rgb_u8(0, 0, 255),
            Color::from_rgb_u8(255, 0, 0),
            Color::from_rgb_u8(255, 255, 0),
            Color::from_rgb_u8(255, 0, 0),
            Color::from_rgb_u8(0, 0, 255),
        ])
        .domain(&[0., 23.])
        .build()
        .unwrap();

    let temp_grad = CustomGradient::new()
        .colors(&[
            Color::from_rgb_u8(255, 255, 255),
            Color::from_rgb_u8(66, 66, 255),
            Color::from_rgb_u8(66, 255, 66),
            Color::from_rgb_u8(255, 166, 66),
        ])
        .domain(&[0., 32., 72., 84.])
        .build()
        .unwrap();

    // Store sunrise/sunset times for each new date
    let mut sun_date = (None, None, None);
    for period in forecast.properties.periods {
        // Compute sunrise/sunset
        let start_time: DateTime<Utc> = DateTime::from_utc(period.start_time.naive_utc(), Utc);
        let date = start_time.date();
        if sun_date.0.unwrap_or_else(|| Utc.timestamp(0, 0).date()) != date {
            let (sunrise, sunset) = sunrise_sunset(LAT, LON, date.year(), date.month(), date.day());
            let sunrise = Utc.timestamp(sunrise, 0);
            let sunset = Utc.timestamp(sunset, 0);
            sun_date = (Some(date), Some(sunrise), Some(sunset));
        }

        // Format time for display
        let time_color = if (start_time > sun_date.1.unwrap() && start_time < sun_date.2.unwrap())
            || start_time.hour() == sun_date.1.unwrap().hour()
            || start_time.hour() == sun_date.2.unwrap().hour()
        {
            style::Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            }
        } else {
            style::Color::Rgb {
                r: 100,
                g: 0,
                b: 255,
            }
        };
        let local = Local.from_utc_datetime(&start_time.naive_utc());
        let time = local.format("%a %l%P").to_string();

        // Format temperature for display
        let (r, g, b, _a) = temp_grad.at(period.temperature as f64).rgba_u8();
        let temp_color = style::Color::Rgb { r, g, b };
        let temp = format!("{}°{}", period.temperature, period.temperature_unit);

        println!(
            "{0} {1} {2: >3$} {4: <2} {5}",
            time.with(time_color),
            temp.with(temp_color),
            period.wind_speed,
            max_wind_column_len,
            period.wind_direction,
            period.short_forecast
        );
    }
}

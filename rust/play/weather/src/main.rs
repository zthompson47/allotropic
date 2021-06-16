use chrono::{Datelike, Duration, Timelike};
use colorgrad::{Color, CustomGradient};
use crossterm::style::{self, Stylize};
use sunrise::sunrise_sunset;
use weather::client::ApiClient;

const API: &str = "https://api.weather.gov";
const APP: &str = "weather.allotropic.com";
const USER: &str = "zach@allotropic.com";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut client = ApiClient::new(API, APP, USER).unwrap();

    let lat: f64 = 42.440932990492946;
    let lon: f64 = -76.52462385924595;
    let point = client.get_point(vec![lat, lon]).await.unwrap();

    let forecast = client
        .get_forecast_from_url(point.properties.forecast_hourly)
        .await
        .unwrap();

    let max_wind_speed_len = forecast
        .properties
        .periods
        .iter()
        .map(|x| x.wind_speed.len())
        .max()
        .unwrap();

    let time_grad = CustomGradient::new()
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
            Color::from_rgb_u8(0, 0, 200),
            Color::from_rgb_u8(0, 0, 255),
            Color::from_rgb_u8(0, 255, 0),
            Color::from_rgb_u8(255, 255, 0),
        ])
        .domain(&[0., 32., 72., 90.])
        .build()
        .unwrap();

    let mut sun_date = (None, None, None);
    for period in forecast.properties.periods {
        // Date for sunrise/sunset
        let date = period.start_time.date();
        if sun_date.0.is_none() {
            sun_date.0 = Some(date);
        }
        if sun_date.0.unwrap() != date {
            let (sunrise, sunset) = sunrise_sunset(lat, lon, date.year(), date.month(), date.day());
            sun_date = (
                Some(date),
                Some(Duration::milliseconds(sunrise)),
                Some(Duration::milliseconds(sunset)),
            );
            println!("{:?}", sun_date);
        }

        // Time
        let hour = period.start_time.hour() as f64;
        let (r, g, b, _a) = time_grad.at(hour).rgba_u8();
        let time_color = style::Color::Rgb { r, g, b };
        let time = period.start_time.format("%a %l%P").to_string();
        //let time = time.with(style::Color::Rgb { r, g, b });

        // Temperature
        let (r, g, b, _a) = temp_grad.at(period.temperature as f64).rgba_u8();
        let temp_color = style::Color::Rgb { r, g, b };
        let temp = format!("{}°{}", period.temperature, period.temperature_unit);

        //print!("{} ", time.with(time_color));
        println!(
            "{0} {1} {2: >3$} {4: <2} {5}",
            time.with(time_color),
            temp.with(temp_color),
            period.wind_speed,
            max_wind_speed_len,
            period.wind_direction,
            period.short_forecast
        );
    }
}

use weather::client::ApiClient;

const API: &str = "https://api.weather.gov";
const APP: &str = "weather.allotropic.com";
const USER: &str = "zach@allotropic.com";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut client = ApiClient::new(API, APP, USER).unwrap();

    let latitude: f64 = 42.440932990492946;
    let longitude: f64 = -76.52462385924595;
    let point = client.get_point(vec![latitude, longitude]).await.unwrap();

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

    for period in forecast.properties.periods {
        print!("{} ", period.start_time.format("%a %l%P"));
        println!(
            "{0}°{1} {2: >3$} {4: <2} {5}",
            period.temperature,
            period.temperature_unit,
            period.wind_speed,
            max_wind_speed_len,
            period.wind_direction,
            period.short_forecast
        );
    }
}

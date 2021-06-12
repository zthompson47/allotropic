use weather::client::ApiClient;

const API: &str = "https://api.weather.gov";
const APP: &str = "weather.allotropic.com";
const USER: &str = "zach@allotropic.com";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let latitude: f64 = 42.440932990492946;
    let longitude: f64 = -76.52462385924595;
    let mut client = ApiClient::new(API, APP, USER).unwrap();
    let point = client.get_point(vec![latitude, longitude]).await.unwrap();
    let forecast = client
        .get_forecast_from_url(point.properties.forecast_hourly)
        .await
        .unwrap();
    for period in forecast.properties.periods {
        println!("{}", period.start_time);
        println!(
            "{}{}, {} {}, {}",
            period.temperature,
            period.temperature_unit,
            period.wind_speed,
            period.wind_direction,
            period.short_forecast
        );
    }
    //println!("{:#?}", forecast);
}

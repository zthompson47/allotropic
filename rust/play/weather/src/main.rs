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
    println!("{:#?}", point);
}

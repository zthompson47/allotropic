use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use wthr::client::ApiClient;

mod common;
use common::{json, APP, USER};

#[tokio::test]
async fn get_point() {
    let mock_server = MockServer::start().await;
    let response = ResponseTemplate::new(200)
        .set_body_string(json("get_point"))
        .insert_header("content-type", "application/geo+json");

    Mock::given(method("GET"))
        .and(path("/points/42.4465,-76.4807"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let latitude: f64 = 42.44645644561855;
    let longitude: f64 = -76.4807390759812;

    //let mut client = ApiClient::new(&mock_server.uri(), APP, USER).unwrap();
    let mut client = ApiClient::builder()
        .base_url(&mock_server.uri())
        .api_key(APP, USER)
        .build()
        .unwrap();

    let point = client.get_point(vec![latitude, longitude]).await.unwrap();

    assert_eq!(
        point.properties.relative_location.properties.city,
        "Forest Home"
    );
    assert_eq!((44, 69), (point.properties.grid_x, point.properties.grid_y));
}

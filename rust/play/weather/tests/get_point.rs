mod common;

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use weather::client::{round_fmt, ApiClient};

use common::{json, API, APP, USER};

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
    let mut client = ApiClient::new(&mock_server.uri(), APP, USER).unwrap();
    let point = client.get_point(vec![latitude, longitude]).await.unwrap();

    assert_eq!(
        point.properties.relative_location.properties.city,
        "Forest Home"
    );
    assert_eq!((44, 69), (point.properties.grid_x, point.properties.grid_y));
}

#[tokio::test]
async fn get_point_from_cache() {
    // The place
    let latitude: f64 = 42.44645644561855;
    let longitude: f64 = -76.4807390759812;

    // Credentials
    let app = "test.app";
    let user = "user@test.app";

    // Create cached response
    let coords = format!("{},{}", round_fmt(latitude, 4), round_fmt(longitude, 4));
    let url = format!("{}/points/{}", API, coords);
    let mut client = ApiClient::new(API, app, user).unwrap();
    client.cache.insert(url.clone(), json("get_point"));

    // Run request on cached response
    let point = client.get_point(vec![latitude, longitude]).await.unwrap();
    assert_eq!(point.id, url);
    assert_eq!(
        point.properties.relative_location.properties.city,
        "Forest Home"
    );
    assert_eq!((44, 69), (point.properties.grid_x, point.properties.grid_y));
}

#![allow(unused_imports)]
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use weather::{cache::Cache, client::{round_fmt, ApiClient}};

mod common;
use common::{json, API, APP, USER};

#[tokio::test]
async fn fetch_from_cache() {
    let server = MockServer::start().await;

    let response = ResponseTemplate::new(200)
        .set_body_string(json("get_point"))
        .insert_header("content-type", "application/geo+json");

    Mock::given(method("GET"))
        .and(path("/points/42.4465,-76.4807"))
        .respond_with(response)
        .mount(&server)
        .await;

    let _c = Cache::new().unwrap();
}

#[allow(dead_code)]
pub const API: &str = "https://mock.api";
#[allow(dead_code)]
pub const APP: &str = "test.app";
#[allow(dead_code)]
pub const USER: &str = "user@test.app";

pub fn json(file: &str) -> String {
    let path = format!(
        "{}/tests/json/{}.json",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        file
    );
    std::fs::read_to_string(path).unwrap()
}

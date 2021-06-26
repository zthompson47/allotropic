use wthr::forecast::Forecast;

mod common;
use common::json;

#[test]
fn parse_forecast() {
    let data = json("forecast");
    let _parsed: Forecast = serde_json::from_str(&data).unwrap();
    //println!("{:#?}", parsed);
    //panic!();
}

use weather::forecast::GridpointData;

mod common;
use common::json;

#[test]
fn using_geojson_geometry_type() {
    let data = json("gridpoint_data");
    let parsed: GridpointData = serde_json::from_str(&data).unwrap();
    //println!("{:#?}", parsed);
    //panic!();
    match parsed.geometry.value {
        geojson::Value::Polygon(poly) => assert_eq!(poly[0].len(), 5),
        _ => panic!(),
    }
}

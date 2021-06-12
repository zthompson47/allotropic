use serde::Deserialize;

pub type Position = Vec<f64>;
pub type Url = String;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_of: String, // TODO: should be enum
    pub coordinates: Position,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    pub value: f32,
    pub unit_code: String,
}

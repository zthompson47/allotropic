use serde::Deserialize;

pub type Angle = u32;
pub type TimeInterval = String;
pub type Percent = u8;
pub type Position = Vec<f64>;
pub type Url = String;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    pub value: Option<f64>,
    pub unit_code: String,
}

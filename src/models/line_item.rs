/*
 * eHelply SDK - 1.1.112
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.112
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LineItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    #[serde(rename = "unit_price")]
    pub unit_price: i32,
    #[serde(rename = "total")]
    pub total: i32,
}

impl LineItem {
    pub fn new(name: String, quantity: i32, unit_price: i32, total: i32) -> LineItem {
        LineItem {
            name,
            quantity,
            unit_price,
            total,
        }
    }
}



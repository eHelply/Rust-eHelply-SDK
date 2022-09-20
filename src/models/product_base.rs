/*
 * eHelply SDK - 1.1.109
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.109
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProductBase {
    #[serde(rename = "meta_data", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "collection_uuid", skip_serializing_if = "Option::is_none")]
    pub collection_uuid: Option<String>,
    #[serde(rename = "review_group_uuid", skip_serializing_if = "Option::is_none")]
    pub review_group_uuid: Option<String>,
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "quantity_for_public")]
    pub quantity_for_public: i32,
}

impl ProductBase {
    pub fn new(price: i32, quantity_for_public: i32) -> ProductBase {
        ProductBase {
            meta_data: None,
            collection_uuid: None,
            review_group_uuid: None,
            addons: None,
            name: None,
            price,
            quantity_for_public,
        }
    }
}



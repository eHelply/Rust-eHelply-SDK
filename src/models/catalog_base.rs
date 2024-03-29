/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CatalogBase {
    #[serde(rename = "meta_data", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "featured", skip_serializing_if = "Option::is_none")]
    pub featured: Option<serde_json::Value>,
    #[serde(rename = "sub_catalogs", skip_serializing_if = "Option::is_none")]
    pub sub_catalogs: Option<serde_json::Value>,
}

impl CatalogBase {
    pub fn new() -> CatalogBase {
        CatalogBase {
            meta_data: None,
            name: None,
            featured: None,
            sub_catalogs: None,
        }
    }
}



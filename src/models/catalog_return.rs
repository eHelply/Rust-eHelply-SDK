/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CatalogReturn {
    #[serde(rename = "meta_data", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "featured", skip_serializing_if = "Option::is_none")]
    pub featured: Option<serde_json::Value>,
    #[serde(rename = "sub_catalogs", skip_serializing_if = "Option::is_none")]
    pub sub_catalogs: Option<serde_json::Value>,
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "meta_uuid", skip_serializing_if = "Option::is_none")]
    pub meta_uuid: Option<String>,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "product_uuids", skip_serializing_if = "Option::is_none")]
    pub product_uuids: Option<Vec<String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl CatalogReturn {
    pub fn new(uuid: String, project_uuid: String) -> CatalogReturn {
        CatalogReturn {
            meta_data: None,
            name: None,
            featured: None,
            sub_catalogs: None,
            uuid,
            meta_uuid: None,
            project_uuid,
            product_uuids: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}


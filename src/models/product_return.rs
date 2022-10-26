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
pub struct ProductReturn {
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
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "meta_uuid", skip_serializing_if = "Option::is_none")]
    pub meta_uuid: Option<String>,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "catalog_uuid", skip_serializing_if = "Option::is_none")]
    pub catalog_uuid: Option<String>,
    #[serde(rename = "addon_list", skip_serializing_if = "Option::is_none")]
    pub addon_list: Option<Vec<serde_json::Value>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl ProductReturn {
    pub fn new(price: i32, quantity_for_public: i32, uuid: String, project_uuid: String) -> ProductReturn {
        ProductReturn {
            meta_data: None,
            collection_uuid: None,
            review_group_uuid: None,
            addons: None,
            name: None,
            price,
            quantity_for_public,
            uuid,
            meta_uuid: None,
            project_uuid,
            catalog_uuid: None,
            addon_list: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}



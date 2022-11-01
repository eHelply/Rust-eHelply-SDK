/*
 * eHelply SDK - 1.1.121
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.121
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// CategoryBase : **:param** name                                **type:** string **:param** meta                                **type:** dict or None  **:param** project_uuid                        **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryBase {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "meta_uuid", skip_serializing_if = "Option::is_none")]
    pub meta_uuid: Option<String>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
}

impl CategoryBase {
    /// **:param** name                                **type:** string **:param** meta                                **type:** dict or None  **:param** project_uuid                        **type:** string or None
    pub fn new(name: String) -> CategoryBase {
        CategoryBase {
            name,
            meta_uuid: None,
            meta: None,
            project_uuid: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.111
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.111
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// CategoryDb : **:param** uuid                                **type:** string **:param** name                                **type:** string **:param** project_uuid                        **type:** string or None  **:param** meta_uuid                           **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryDb {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
    #[serde(rename = "meta_uuid", skip_serializing_if = "Option::is_none")]
    pub meta_uuid: Option<String>,
}

impl CategoryDb {
    /// **:param** uuid                                **type:** string **:param** name                                **type:** string **:param** project_uuid                        **type:** string or None  **:param** meta_uuid                           **type:** string or None
    pub fn new(uuid: String, name: String) -> CategoryDb {
        CategoryDb {
            uuid,
            name,
            project_uuid: None,
            meta_uuid: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// TagBase : **:param** name                                **type:** string **:param** project_uuid                        **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TagBase {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
}

impl TagBase {
    /// **:param** name                                **type:** string **:param** project_uuid                        **type:** string or None
    pub fn new(name: String) -> TagBase {
        TagBase {
            name,
            project_uuid: None,
        }
    }
}



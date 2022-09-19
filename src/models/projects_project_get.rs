/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectsProjectGet : Used for get endpoint



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectsProjectGet {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "archived_at", skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

impl ProjectsProjectGet {
    /// Used for get endpoint
    pub fn new(uuid: String, name: String) -> ProjectsProjectGet {
        ProjectsProjectGet {
            uuid,
            name,
            status: None,
            archived_at: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectDb : Used for DB row entry



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectDb {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "current_spend")]
    pub current_spend: i32,
    #[serde(rename = "max_spend")]
    pub max_spend: i32,
    #[serde(rename = "project_status")]
    pub project_status: String,
    #[serde(rename = "archived_at", skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

impl ProjectDb {
    /// Used for DB row entry
    pub fn new(uuid: String, name: String, created_at: String, current_spend: i32, max_spend: i32, project_status: String) -> ProjectDb {
        ProjectDb {
            uuid,
            name,
            created_at,
            current_spend,
            max_spend,
            project_status,
            archived_at: None,
        }
    }
}



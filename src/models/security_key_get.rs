/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityKeyGet : Used for get endpoint



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityKeyGet {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "access")]
    pub access: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_used_at")]
    pub last_used_at: String,
}

impl SecurityKeyGet {
    /// Used for get endpoint
    pub fn new(uuid: String, access: String, name: String, summary: String, created_at: String, last_used_at: String) -> SecurityKeyGet {
        SecurityKeyGet {
            uuid,
            access,
            name,
            summary,
            created_at,
            last_used_at,
        }
    }
}


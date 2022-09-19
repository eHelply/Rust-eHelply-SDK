/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ParticipantCreate : Contains field for when we create a participant only



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParticipantCreate {
    /// Dictionary containing any data you would like
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "user_uuid", skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<String>,
}

impl ParticipantCreate {
    /// Contains field for when we create a participant only
    pub fn new() -> ParticipantCreate {
        ParticipantCreate {
            meta: None,
            user_uuid: None,
        }
    }
}



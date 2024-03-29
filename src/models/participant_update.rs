/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ParticipantUpdate : Fields for updating participants including the participant uuid



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParticipantUpdate {
    /// Dictionary containing any data you would like
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "user_uuid", skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl ParticipantUpdate {
    /// Fields for updating participants including the participant uuid
    pub fn new(uuid: String) -> ParticipantUpdate {
        ParticipantUpdate {
            meta: None,
            user_uuid: None,
            uuid,
        }
    }
}



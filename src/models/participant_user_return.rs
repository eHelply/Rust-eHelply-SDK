/*
 * eHelply SDK - 1.1.112
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.112
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ParticipantUserReturn : Contains all fields required when doing a Participant GET but also has user fields (name, location, ect). This is what is returned from all participant endpoints.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParticipantUserReturn {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "user_uuid", skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<String>,
    #[serde(rename = "participant_meta", skip_serializing_if = "Option::is_none")]
    pub participant_meta: Option<serde_json::Value>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<crate::models::Email>>,
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "gps_location", skip_serializing_if = "Option::is_none")]
    pub gps_location: Option<serde_json::Value>,
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(rename = "last_login", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    #[serde(rename = "verified_legal_terms", skip_serializing_if = "Option::is_none")]
    pub verified_legal_terms: Option<bool>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
}

impl ParticipantUserReturn {
    /// Contains all fields required when doing a Participant GET but also has user fields (name, location, ect). This is what is returned from all participant endpoints.
    pub fn new() -> ParticipantUserReturn {
        ParticipantUserReturn {
            uuid: None,
            user_uuid: None,
            participant_meta: None,
            first_name: None,
            last_name: None,
            email: None,
            phone_number: None,
            country: None,
            gps_location: None,
            picture: None,
            last_login: None,
            verified_legal_terms: None,
            date_created: None,
            date_updated: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.113
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.113
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserResponse : When retrieving a user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(rename = "gps_location", skip_serializing_if = "Option::is_none")]
    pub gps_location: Option<serde_json::Value>,
    #[serde(rename = "verified_legal_terms", skip_serializing_if = "Option::is_none")]
    pub verified_legal_terms: Option<bool>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "date_deleted", skip_serializing_if = "Option::is_none")]
    pub date_deleted: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<crate::models::UserEmail>>,
    #[serde(rename = "missing", skip_serializing_if = "Option::is_none")]
    pub missing: Option<Vec<String>>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<serde_json::Value>>,
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Box<crate::models::UserFlags>>,
    #[serde(rename = "last_login", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
}

impl UserResponse {
    /// When retrieving a user
    pub fn new() -> UserResponse {
        UserResponse {
            first_name: None,
            last_name: None,
            phone_number: None,
            country: None,
            picture: None,
            gps_location: None,
            verified_legal_terms: None,
            date_created: None,
            date_updated: None,
            date_deleted: None,
            email: None,
            missing: None,
            uuid: None,
            participants: None,
            flags: None,
            last_login: None,
        }
    }
}



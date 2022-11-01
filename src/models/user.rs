/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// User : User object, contains all required parameters



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "cognito_id", skip_serializing_if = "Option::is_none")]
    pub cognito_id: Option<String>,
    #[serde(rename = "cognito_data", skip_serializing_if = "Option::is_none")]
    pub cognito_data: Option<serde_json::Value>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
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
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "last_login", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    #[serde(rename = "first_login", skip_serializing_if = "Option::is_none")]
    pub first_login: Option<bool>,
}

impl User {
    /// User object, contains all required parameters
    pub fn new(uuid: String) -> User {
        User {
            cognito_id: None,
            cognito_data: None,
            first_name: None,
            last_name: None,
            email: None,
            phone_number: None,
            country: None,
            picture: None,
            gps_location: None,
            verified_legal_terms: None,
            date_created: None,
            date_updated: None,
            date_deleted: None,
            uuid,
            last_login: None,
            first_login: None,
        }
    }
}



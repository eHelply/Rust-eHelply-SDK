/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserSignup : User information used for user signup



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserSignup {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "phone_number")]
    pub phone_number: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<String>,
    #[serde(rename = "lng", skip_serializing_if = "Option::is_none")]
    pub lng: Option<String>,
    #[serde(rename = "verified_legal_terms", skip_serializing_if = "Option::is_none")]
    pub verified_legal_terms: Option<bool>,
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(rename = "newsletters", skip_serializing_if = "Option::is_none")]
    pub newsletters: Option<bool>,
}

impl UserSignup {
    /// User information used for user signup
    pub fn new(username: String, password: String, email: String, first_name: String, last_name: String, phone_number: String, country: String) -> UserSignup {
        UserSignup {
            username,
            password,
            email,
            first_name,
            last_name,
            phone_number,
            country,
            lat: None,
            lng: None,
            verified_legal_terms: None,
            picture: None,
            newsletters: None,
        }
    }
}



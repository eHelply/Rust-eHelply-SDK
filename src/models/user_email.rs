/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserEmail : User email information



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserEmail {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl UserEmail {
    /// User email information
    pub fn new() -> UserEmail {
        UserEmail {
            address: None,
            verified: None,
        }
    }
}


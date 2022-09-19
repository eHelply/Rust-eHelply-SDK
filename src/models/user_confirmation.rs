/*
 * eHelply SDK - 1.1.114
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.114
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserConfirmation : Information to confirm user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserConfirmation {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "verification_code")]
    pub verification_code: String,
}

impl UserConfirmation {
    /// Information to confirm user
    pub fn new(email: String, verification_code: String) -> UserConfirmation {
        UserConfirmation {
            email,
            verification_code,
        }
    }
}



/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserPasswordResetConfirmation : Information for resetting the new password with a confirmation code



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserPasswordResetConfirmation {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "confirmation_code")]
    pub confirmation_code: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl UserPasswordResetConfirmation {
    /// Information for resetting the new password with a confirmation code
    pub fn new(email: String, confirmation_code: String, password: String) -> UserPasswordResetConfirmation {
        UserPasswordResetConfirmation {
            email,
            confirmation_code,
            password,
        }
    }
}



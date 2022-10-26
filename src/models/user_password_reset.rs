/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserPasswordReset : Information used for resetting the password



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserPasswordReset {
    #[serde(rename = "email")]
    pub email: String,
}

impl UserPasswordReset {
    /// Information used for resetting the password
    pub fn new(email: String) -> UserPasswordReset {
        UserPasswordReset {
            email,
        }
    }
}



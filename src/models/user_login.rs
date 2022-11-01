/*
 * eHelply SDK - 1.1.121
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.121
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserLogin : Password and username required to login



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserLogin {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl UserLogin {
    /// Password and username required to login
    pub fn new(username: String, password: String) -> UserLogin {
        UserLogin {
            username,
            password,
        }
    }
}



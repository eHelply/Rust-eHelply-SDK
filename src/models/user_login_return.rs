/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserLoginReturn : Tokens (refresh, access, identity, ect)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserLoginReturn {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "ExpiresIn")]
    pub expires_in: i32,
    #[serde(rename = "TokenType")]
    pub token_type: String,
    #[serde(rename = "IdToken")]
    pub id_token: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
}

impl UserLoginReturn {
    /// Tokens (refresh, access, identity, ect)
    pub fn new(access_token: String, expires_in: i32, token_type: String, id_token: String, refresh_token: String) -> UserLoginReturn {
        UserLoginReturn {
            access_token,
            expires_in,
            token_type,
            id_token,
            refresh_token,
        }
    }
}



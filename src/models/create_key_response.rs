/*
 * eHelply SDK - 1.1.122
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.122
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "access")]
    pub access: String,
    #[serde(rename = "secret")]
    pub secret: String,
}

impl CreateKeyResponse {
    pub fn new(uuid: String, access: String, secret: String) -> CreateKeyResponse {
        CreateKeyResponse {
            uuid,
            access,
            secret,
        }
    }
}



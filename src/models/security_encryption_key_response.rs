/*
 * eHelply SDK - 1.1.121
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.121
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityEncryptionKeyResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "key")]
    pub key: std::path::PathBuf,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "deleted_at")]
    pub deleted_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "retrieved_at")]
    pub retrieved_at: String,
}

impl SecurityEncryptionKeyResponse {
    pub fn new(uuid: String, key: std::path::PathBuf, category: String, deleted_at: String, created_at: String, retrieved_at: String) -> SecurityEncryptionKeyResponse {
        SecurityEncryptionKeyResponse {
            uuid,
            key,
            category,
            deleted_at,
            created_at,
            retrieved_at,
        }
    }
}



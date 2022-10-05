/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityEncryptionKeyGet {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "retrieved_at")]
    pub retrieved_at: String,
}

impl SecurityEncryptionKeyGet {
    pub fn new(key: String, category: String, created_at: String, retrieved_at: String) -> SecurityEncryptionKeyGet {
        SecurityEncryptionKeyGet {
            key,
            category,
            created_at,
            retrieved_at,
        }
    }
}



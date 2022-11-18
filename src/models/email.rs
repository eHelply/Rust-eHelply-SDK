/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Email {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl Email {
    pub fn new() -> Email {
        Email {
            address: None,
            verified: None,
        }
    }
}



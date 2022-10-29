/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityCreateToken {
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
}

impl SecurityCreateToken {
    pub fn new() -> SecurityCreateToken {
        SecurityCreateToken {
            length: None,
        }
    }
}



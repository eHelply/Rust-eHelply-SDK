/*
 * eHelply SDK - 1.1.108
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.108
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContactMethod {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ContactMethod {
    pub fn new() -> ContactMethod {
        ContactMethod {
            name: None,
            value: None,
        }
    }
}



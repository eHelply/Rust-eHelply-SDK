/*
 * eHelply SDK - 1.1.112
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.112
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseCreateprojectinvoice {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ResponseCreateprojectinvoice {
    pub fn new() -> ResponseCreateprojectinvoice {
        ResponseCreateprojectinvoice {
            message: None,
        }
    }
}



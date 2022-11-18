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
pub struct TouchMeta200Response {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl TouchMeta200Response {
    pub fn new() -> TouchMeta200Response {
        TouchMeta200Response {
            message: None,
        }
    }
}



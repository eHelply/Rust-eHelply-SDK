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
pub struct DeleteField200Response {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DeleteField200Response {
    pub fn new() -> DeleteField200Response {
        DeleteField200Response {
            message: None,
        }
    }
}



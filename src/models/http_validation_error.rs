/*
 * eHelply SDK - 1.1.119
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.119
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HttpValidationError {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<crate::models::ValidationError>>,
}

impl HttpValidationError {
    pub fn new() -> HttpValidationError {
        HttpValidationError {
            detail: None,
        }
    }
}



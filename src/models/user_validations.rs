/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserValidations : Fields used to validate a user's field



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserValidations {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "validation_type")]
    pub validation_type: String,
}

impl UserValidations {
    /// Fields used to validate a user's field
    pub fn new(value: String, validation_type: String) -> UserValidations {
        UserValidations {
            value,
            validation_type,
        }
    }
}



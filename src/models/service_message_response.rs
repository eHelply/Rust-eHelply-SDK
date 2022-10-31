/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceMessageResponse {
    #[serde(rename = "message")]
    pub message: String,
}

impl ServiceMessageResponse {
    pub fn new(message: String) -> ServiceMessageResponse {
        ServiceMessageResponse {
            message,
        }
    }
}



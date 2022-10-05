/*
 * eHelply SDK - 1.1.111
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.111
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetServiceSpecResponse {
    #[serde(rename = "contents")]
    pub contents: String,
}

impl GetServiceSpecResponse {
    pub fn new(contents: String) -> GetServiceSpecResponse {
        GetServiceSpecResponse {
            contents,
        }
    }
}



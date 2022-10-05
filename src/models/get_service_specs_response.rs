/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetServiceSpecsResponse {
    #[serde(rename = "specs")]
    pub specs: Vec<String>,
}

impl GetServiceSpecsResponse {
    pub fn new(specs: Vec<String>) -> GetServiceSpecsResponse {
        GetServiceSpecsResponse {
            specs,
        }
    }
}



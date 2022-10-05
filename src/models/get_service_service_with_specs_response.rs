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
pub struct GetServiceServiceWithSpecsResponse {
    #[serde(rename = "services")]
    pub services: Vec<String>,
}

impl GetServiceServiceWithSpecsResponse {
    pub fn new(services: Vec<String>) -> GetServiceServiceWithSpecsResponse {
        GetServiceServiceWithSpecsResponse {
            services,
        }
    }
}



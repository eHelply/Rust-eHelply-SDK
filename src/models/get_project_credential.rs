/*
 * eHelply SDK - 1.1.113
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.113
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetProjectCredential {
    #[serde(rename = "service_name")]
    pub service_name: String,
    #[serde(rename = "secrets")]
    pub secrets: Vec<crate::models::GetSecret>,
}

impl GetProjectCredential {
    pub fn new(service_name: String, secrets: Vec<crate::models::GetSecret>) -> GetProjectCredential {
        GetProjectCredential {
            service_name,
            secrets,
        }
    }
}



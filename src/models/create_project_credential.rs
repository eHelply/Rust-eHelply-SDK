/*
 * eHelply SDK - 1.1.120
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.120
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateProjectCredential {
    #[serde(rename = "service_name")]
    pub service_name: String,
    #[serde(rename = "secrets")]
    pub secrets: Vec<crate::models::Credential>,
}

impl CreateProjectCredential {
    pub fn new(service_name: String, secrets: Vec<crate::models::Credential>) -> CreateProjectCredential {
        CreateProjectCredential {
            service_name,
            secrets,
        }
    }
}



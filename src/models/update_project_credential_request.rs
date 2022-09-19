/*
 * eHelply SDK - 1.1.108
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.108
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateProjectCredentialRequest {
    #[serde(rename = "secrets")]
    pub secrets: Vec<crate::models::Credential>,
}

impl UpdateProjectCredentialRequest {
    pub fn new(secrets: Vec<crate::models::Credential>) -> UpdateProjectCredentialRequest {
        UpdateProjectCredentialRequest {
            secrets,
        }
    }
}



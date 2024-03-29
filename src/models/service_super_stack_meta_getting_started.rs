/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceSuperStackMetaGettingStarted {
    #[serde(rename = "blurb")]
    pub blurb: String,
    #[serde(rename = "endpoint_teasers")]
    pub endpoint_teasers: Vec<crate::models::ServiceSuperStackMetaGettingStartedEndpointTeaser>,
}

impl ServiceSuperStackMetaGettingStarted {
    pub fn new(blurb: String, endpoint_teasers: Vec<crate::models::ServiceSuperStackMetaGettingStartedEndpointTeaser>) -> ServiceSuperStackMetaGettingStarted {
        ServiceSuperStackMetaGettingStarted {
            blurb,
            endpoint_teasers,
        }
    }
}



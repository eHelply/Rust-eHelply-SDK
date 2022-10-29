/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StripeCustomerSecretResponse {
    #[serde(rename = "secret")]
    pub secret: String,
}

impl StripeCustomerSecretResponse {
    pub fn new(secret: String) -> StripeCustomerSecretResponse {
        StripeCustomerSecretResponse {
            secret,
        }
    }
}



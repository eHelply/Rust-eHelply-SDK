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
pub struct StripeAccountResponse {
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "stripe_customer_id")]
    pub stripe_customer_id: String,
}

impl StripeAccountResponse {
    pub fn new(first_name: String, last_name: String, project_uuid: String, stripe_customer_id: String) -> StripeAccountResponse {
        StripeAccountResponse {
            first_name,
            last_name,
            project_uuid,
            stripe_customer_id,
        }
    }
}



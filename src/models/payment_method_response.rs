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
pub struct PaymentMethodResponse {
    #[serde(rename = "payment_id")]
    pub payment_id: String,
    #[serde(rename = "last_4_digits")]
    pub last_4_digits: String,
    #[serde(rename = "card_brand")]
    pub card_brand: String,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
}

impl PaymentMethodResponse {
    pub fn new(payment_id: String, last_4_digits: String, card_brand: String, project_uuid: String) -> PaymentMethodResponse {
        PaymentMethodResponse {
            payment_id,
            last_4_digits,
            card_brand,
            project_uuid,
        }
    }
}



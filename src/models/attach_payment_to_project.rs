/*
 * eHelply SDK - 1.1.109
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.109
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachPaymentToProject {
    #[serde(rename = "payment_type")]
    pub payment_type: String,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "exp_month")]
    pub exp_month: i32,
    #[serde(rename = "exp_year")]
    pub exp_year: i32,
    #[serde(rename = "cvc")]
    pub cvc: String,
}

impl AttachPaymentToProject {
    pub fn new(payment_type: String, number: String, exp_month: i32, exp_year: i32, cvc: String) -> AttachPaymentToProject {
        AttachPaymentToProject {
            payment_type,
            number,
            exp_month,
            exp_year,
            cvc,
        }
    }
}



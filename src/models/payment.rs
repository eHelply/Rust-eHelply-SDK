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
pub struct Payment {
    #[serde(rename = "amount")]
    pub amount: i32,
}

impl Payment {
    pub fn new(amount: i32) -> Payment {
        Payment {
            amount,
        }
    }
}



/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContactResponse {
    #[serde(rename = "contact_id")]
    pub contact_id: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phone")]
    pub phone: String,
}

impl ContactResponse {
    pub fn new(contact_id: String, first_name: String, last_name: String, email: String, phone: String) -> ContactResponse {
        ContactResponse {
            contact_id,
            first_name,
            last_name,
            email,
            phone,
        }
    }
}



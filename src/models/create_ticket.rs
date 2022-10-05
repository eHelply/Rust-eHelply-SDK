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
pub struct CreateTicket {
    #[serde(rename = "priority")]
    pub priority: String,
    #[serde(rename = "subject")]
    pub subject: String,
}

impl CreateTicket {
    pub fn new(priority: String, subject: String) -> CreateTicket {
        CreateTicket {
            priority,
            subject,
        }
    }
}



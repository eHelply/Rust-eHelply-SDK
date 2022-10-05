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
pub struct TicketResponse {
    #[serde(rename = "ticket_id")]
    pub ticket_id: String,
    #[serde(rename = "contact_id")]
    pub contact_id: String,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "priority")]
    pub priority: String,
}

impl TicketResponse {
    pub fn new(ticket_id: String, contact_id: String, subject: String, priority: String) -> TicketResponse {
        TicketResponse {
            ticket_id,
            contact_id,
            subject,
            priority,
        }
    }
}



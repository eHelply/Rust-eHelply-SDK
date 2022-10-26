/*
 * eHelply SDK - 1.1.112
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.112
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TicketsResponse {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "priority")]
    pub priority: String,
    #[serde(rename = "ticket_id")]
    pub ticket_id: String,
}

impl TicketsResponse {
    pub fn new(subject: String, priority: String, ticket_id: String) -> TicketsResponse {
        TicketsResponse {
            subject,
            priority,
            ticket_id,
        }
    }
}



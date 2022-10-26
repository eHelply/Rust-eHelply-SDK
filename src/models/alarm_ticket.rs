/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AlarmTicket {
    #[serde(rename = "ticket_uuid")]
    pub ticket_uuid: String,
}

impl AlarmTicket {
    pub fn new(ticket_uuid: String) -> AlarmTicket {
        AlarmTicket {
            ticket_uuid,
        }
    }
}



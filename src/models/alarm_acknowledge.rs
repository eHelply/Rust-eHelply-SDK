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
pub struct AlarmAcknowledge {
    #[serde(rename = "acknowledger_uuid")]
    pub acknowledger_uuid: String,
}

impl AlarmAcknowledge {
    pub fn new(acknowledger_uuid: String) -> AlarmAcknowledge {
        AlarmAcknowledge {
            acknowledger_uuid,
        }
    }
}



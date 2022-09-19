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
pub struct AlarmTerminate {
    #[serde(rename = "terminater_uuid")]
    pub terminater_uuid: String,
}

impl AlarmTerminate {
    pub fn new(terminater_uuid: String) -> AlarmTerminate {
        AlarmTerminate {
            terminater_uuid,
        }
    }
}



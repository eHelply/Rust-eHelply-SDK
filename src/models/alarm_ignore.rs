/*
 * eHelply SDK - 1.1.121
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.121
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AlarmIgnore {
    #[serde(rename = "ignorer_uuid")]
    pub ignorer_uuid: String,
}

impl AlarmIgnore {
    pub fn new(ignorer_uuid: String) -> AlarmIgnore {
        AlarmIgnore {
            ignorer_uuid,
        }
    }
}



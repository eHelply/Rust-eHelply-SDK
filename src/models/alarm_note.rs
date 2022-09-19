/*
 * eHelply SDK - 1.1.114
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.114
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AlarmNote {
    #[serde(rename = "author_uuid")]
    pub author_uuid: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl AlarmNote {
    pub fn new(author_uuid: String, message: String) -> AlarmNote {
        AlarmNote {
            author_uuid,
            message,
        }
    }
}



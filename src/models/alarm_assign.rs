/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AlarmAssign {
    #[serde(rename = "assignee_uuid")]
    pub assignee_uuid: String,
}

impl AlarmAssign {
    pub fn new(assignee_uuid: String) -> AlarmAssign {
        AlarmAssign {
            assignee_uuid,
        }
    }
}



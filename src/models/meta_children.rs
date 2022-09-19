/*
 * eHelply SDK - 1.1.108
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.108
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaChildren {
    #[serde(rename = "child_name", skip_serializing_if = "Option::is_none")]
    pub child_name: Option<String>,
    #[serde(rename = "child_description", skip_serializing_if = "Option::is_none")]
    pub child_description: Option<String>,
    #[serde(rename = "child_uuid", skip_serializing_if = "Option::is_none")]
    pub child_uuid: Option<String>,
}

impl MetaChildren {
    pub fn new() -> MetaChildren {
        MetaChildren {
            child_name: None,
            child_description: None,
            child_uuid: None,
        }
    }
}



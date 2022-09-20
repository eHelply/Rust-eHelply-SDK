/*
 * eHelply SDK - 1.1.109
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.109
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Detailed {
    #[serde(rename = "summary_uuid", skip_serializing_if = "Option::is_none")]
    pub summary_uuid: Option<String>,
    #[serde(rename = "description_uuid", skip_serializing_if = "Option::is_none")]
    pub description_uuid: Option<String>,
}

impl Detailed {
    pub fn new() -> Detailed {
        Detailed {
            summary_uuid: None,
            description_uuid: None,
        }
    }
}



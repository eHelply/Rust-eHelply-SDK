/*
 * eHelply SDK - 1.1.119
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.119
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// DetailedMeta : Detailed meta based on Notes



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetailedMeta {
    #[serde(rename = "summary_uuid", skip_serializing_if = "Option::is_none")]
    pub summary_uuid: Option<String>,
    #[serde(rename = "description_uuid", skip_serializing_if = "Option::is_none")]
    pub description_uuid: Option<String>,
}

impl DetailedMeta {
    /// Detailed meta based on Notes
    pub fn new() -> DetailedMeta {
        DetailedMeta {
            summary_uuid: None,
            description_uuid: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// DetailedMetaCreate : Detailed meta based on Notes



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetailedMetaCreate {
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DetailedMetaCreate {
    /// Detailed meta based on Notes
    pub fn new() -> DetailedMetaCreate {
        DetailedMetaCreate {
            summary: None,
            description: None,
        }
    }
}



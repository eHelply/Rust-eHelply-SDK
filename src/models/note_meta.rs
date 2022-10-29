/*
 * eHelply SDK - 1.1.119
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.119
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// NoteMeta : Metadata associated to a note



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NoteMeta {
    #[serde(rename = "original_author", skip_serializing_if = "Option::is_none")]
    pub original_author: Option<String>,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "previous_version", skip_serializing_if = "Option::is_none")]
    pub previous_version: Option<String>,
    #[serde(rename = "next_version", skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
}

impl NoteMeta {
    /// Metadata associated to a note
    pub fn new(author: String) -> NoteMeta {
        NoteMeta {
            original_author: None,
            author,
            previous_version: None,
            next_version: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.109
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.109
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// BasicMeta : Basic meta



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BasicMeta {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl BasicMeta {
    /// Basic meta
    pub fn new() -> BasicMeta {
        BasicMeta {
            name: None,
            slug: None,
        }
    }
}



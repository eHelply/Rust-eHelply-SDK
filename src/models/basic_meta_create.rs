/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// BasicMetaCreate : Basic meta



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BasicMetaCreate {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<bool>,
}

impl BasicMetaCreate {
    /// Basic meta
    pub fn new() -> BasicMetaCreate {
        BasicMetaCreate {
            name: None,
            slug: None,
        }
    }
}



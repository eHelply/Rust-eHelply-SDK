/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// MetaCreate : Meta



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaCreate {
    #[serde(rename = "basic", skip_serializing_if = "Option::is_none")]
    pub basic: Option<Box<crate::models::BasicMetaCreate>>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<Box<crate::models::DetailedMetaCreate>>,
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Box<crate::models::MetaCustom>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::Field>>,
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::MetaChildren>>,
    #[serde(rename = "parent_uuid", skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<String>,
}

impl MetaCreate {
    /// Meta
    pub fn new() -> MetaCreate {
        MetaCreate {
            basic: None,
            detailed: None,
            custom: None,
            fields: None,
            children: None,
            parent_uuid: None,
        }
    }
}


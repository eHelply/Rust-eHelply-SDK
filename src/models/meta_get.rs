/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// MetaGet : Meta



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaGet {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "basic")]
    pub basic: Box<crate::models::BasicMeta>,
    #[serde(rename = "detailed")]
    pub detailed: Box<crate::models::DetailedMetaGet>,
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Box<crate::models::MetaCustom>>,
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Box<crate::models::DatesMeta>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::Field>>,
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::MetaChildren>>,
    #[serde(rename = "parent_uuid", skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<String>,
}

impl MetaGet {
    /// Meta
    pub fn new(uuid: String, basic: crate::models::BasicMeta, detailed: crate::models::DetailedMetaGet) -> MetaGet {
        MetaGet {
            uuid,
            basic: Box::new(basic),
            detailed: Box::new(detailed),
            custom: None,
            dates: None,
            fields: None,
            children: None,
            parent_uuid: None,
        }
    }
}


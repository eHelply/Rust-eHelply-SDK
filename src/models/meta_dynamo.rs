/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// MetaDynamo : A meta from DynamoDB



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaDynamo {
    #[serde(rename = "basic", skip_serializing_if = "Option::is_none")]
    pub basic: Option<Box<crate::models::Basic>>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<Box<crate::models::Detailed>>,
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
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl MetaDynamo {
    /// A meta from DynamoDB
    pub fn new(uuid: String) -> MetaDynamo {
        MetaDynamo {
            basic: None,
            detailed: None,
            custom: None,
            dates: None,
            fields: None,
            children: None,
            parent_uuid: None,
            uuid,
        }
    }
}


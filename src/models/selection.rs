/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// Selection : Selection Model



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Selection {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl Selection {
    /// Selection Model
    pub fn new() -> Selection {
        Selection {
            name: None,
            value: None,
            icon: None,
        }
    }
}


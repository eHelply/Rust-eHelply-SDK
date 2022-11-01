/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// Options : Options



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Options {
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "insetLabel", skip_serializing_if = "Option::is_none")]
    pub inset_label: Option<String>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "hint", skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<f32>,
    #[serde(rename = "counter", skip_serializing_if = "Option::is_none")]
    pub counter: Option<bool>,
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "iconPosition", skip_serializing_if = "Option::is_none")]
    pub icon_position: Option<String>,
    #[serde(rename = "selections", skip_serializing_if = "Option::is_none")]
    pub selections: Option<Vec<crate::models::OptionGroup>>,
}

impl Options {
    /// Options
    pub fn new() -> Options {
        Options {
            required: None,
            label: None,
            inset_label: None,
            placeholder: None,
            hint: None,
            icon: None,
            max_length: None,
            counter: None,
            caption: None,
            color: None,
            size: None,
            _type: None,
            icon_position: None,
            selections: None,
        }
    }
}



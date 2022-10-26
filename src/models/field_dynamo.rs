/*
 * eHelply SDK - 1.1.113
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.113
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldDynamo : Field Dynamo



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldDynamo {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<serde_json::Value>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "validations", skip_serializing_if = "Option::is_none")]
    pub validations: Option<serde_json::Value>,
    #[serde(rename = "hint", skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl FieldDynamo {
    /// Field Dynamo
    pub fn new(uuid: String) -> FieldDynamo {
        FieldDynamo {
            _type: None,
            placeholder: None,
            validations: None,
            hint: None,
            icon: None,
            label: None,
            options: None,
            uuid,
        }
    }
}



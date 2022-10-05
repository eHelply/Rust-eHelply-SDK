/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserFlags : Flags that may be attached to a user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserFlags {
    #[serde(rename = "requires_tour", skip_serializing_if = "Option::is_none")]
    pub requires_tour: Option<bool>,
    #[serde(rename = "missing_data", skip_serializing_if = "Option::is_none")]
    pub missing_data: Option<bool>,
    #[serde(rename = "legal_updates", skip_serializing_if = "Option::is_none")]
    pub legal_updates: Option<bool>,
    #[serde(rename = "newsletters", skip_serializing_if = "Option::is_none")]
    pub newsletters: Option<bool>,
}

impl UserFlags {
    /// Flags that may be attached to a user
    pub fn new() -> UserFlags {
        UserFlags {
            requires_tour: None,
            missing_data: None,
            legal_updates: None,
            newsletters: None,
        }
    }
}



/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// CompanyBase : **:param** name                                **type:** string **:param** summary                             **type:** string or None  **:param** public                              **type:** bool or None  **:param** meta                                **type:** dict or None  **:param** contact                             **type:** ContactBase or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CompanyBase {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<crate::models::ContactBase>>,
}

impl CompanyBase {
    /// **:param** name                                **type:** string **:param** summary                             **type:** string or None  **:param** public                              **type:** bool or None  **:param** meta                                **type:** dict or None  **:param** contact                             **type:** ContactBase or None
    pub fn new() -> CompanyBase {
        CompanyBase {
            name: None,
            summary: None,
            public: None,
            meta: None,
            contact: None,
        }
    }
}


/*
 * eHelply SDK - 1.1.113
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.113
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// PlaceBase : **:param** name                                **type:** string **:param** summary                             **type:** string or None  **:param** public                              **type:** bool or None  **:param** meta                                **type:** dict or None  **:param** addresses                           **type:** PlaceAddress or None  **:param** contact                             **type:** ContactBase or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlaceBase {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::AddressBase>>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<crate::models::ContactBase>>,
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

impl PlaceBase {
    /// **:param** name                                **type:** string **:param** summary                             **type:** string or None  **:param** public                              **type:** bool or None  **:param** meta                                **type:** dict or None  **:param** addresses                           **type:** PlaceAddress or None  **:param** contact                             **type:** ContactBase or None
    pub fn new(name: String) -> PlaceBase {
        PlaceBase {
            name,
            summary: None,
            public: None,
            meta: None,
            addresses: None,
            contact: None,
            picture: None,
        }
    }
}



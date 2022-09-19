/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// PlaceResponse : **:param** name                                **type:** string **:param** summary                             **type:** string or None  **:param** public                              **type:** bool or None  **:param** meta                                **type:** dict or None  **:param** addresses                           **type:** PlaceAddress or None  **:param** contact                             **type:** ContactBase or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlaceResponse {
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
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
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "meta_uuid", skip_serializing_if = "Option::is_none")]
    pub meta_uuid: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::TagBase>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<crate::models::CategoryBase>>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<crate::models::Company>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl PlaceResponse {
    /// **:param** name                                **type:** string **:param** summary                             **type:** string or None  **:param** public                              **type:** bool or None  **:param** meta                                **type:** dict or None  **:param** addresses                           **type:** PlaceAddress or None  **:param** contact                             **type:** ContactBase or None
    pub fn new(name: String, uuid: String) -> PlaceResponse {
        PlaceResponse {
            project_uuid: None,
            name,
            summary: None,
            public: None,
            meta: None,
            addresses: None,
            contact: None,
            picture: None,
            uuid,
            meta_uuid: None,
            tags: None,
            categories: None,
            company: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}



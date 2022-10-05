/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ContactBase : **:param** phones                              **type:** List[ContactKeys] or None  **:param** email                               **type:** string or None  **:param** website                             **type:** string or None  **:param** socials                             **type:** List[ContactKeys] or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContactBase {
    #[serde(rename = "phones", skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<crate::models::ContactMethod>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "socials", skip_serializing_if = "Option::is_none")]
    pub socials: Option<Vec<crate::models::ContactMethod>>,
}

impl ContactBase {
    /// **:param** phones                              **type:** List[ContactKeys] or None  **:param** email                               **type:** string or None  **:param** website                             **type:** string or None  **:param** socials                             **type:** List[ContactKeys] or None
    pub fn new() -> ContactBase {
        ContactBase {
            phones: None,
            email: None,
            website: None,
            socials: None,
        }
    }
}



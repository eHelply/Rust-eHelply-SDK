/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// Page : Page state



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "items")]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "pagination")]
    pub pagination: Box<crate::models::Pagination>,
}

impl Page {
    /// Page state
    pub fn new(items: Vec<serde_json::Value>, pagination: crate::models::Pagination) -> Page {
        Page {
            items,
            pagination: Box::new(pagination),
        }
    }
}



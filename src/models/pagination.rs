/*
 * eHelply SDK - 1.1.114
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.114
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// Pagination : Pagination state



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(rename = "current_page")]
    pub current_page: i32,
    #[serde(rename = "page_size")]
    pub page_size: i32,
    #[serde(rename = "total_items")]
    pub total_items: i32,
    #[serde(rename = "total_pages")]
    pub total_pages: i32,
    #[serde(rename = "has_previous_page")]
    pub has_previous_page: bool,
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "previous_page", skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<i32>,
    #[serde(rename = "next_page", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<i32>,
}

impl Pagination {
    /// Pagination state
    pub fn new(current_page: i32, page_size: i32, total_items: i32, total_pages: i32, has_previous_page: bool, has_next_page: bool) -> Pagination {
        Pagination {
            current_page,
            page_size,
            total_items,
            total_pages,
            has_previous_page,
            has_next_page,
            previous_page: None,
            next_page: None,
        }
    }
}



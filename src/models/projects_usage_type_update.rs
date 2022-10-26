/*
 * eHelply SDK - 1.1.113
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.113
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectsUsageTypeUpdate : Used for update endpoint



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectsUsageTypeUpdate {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "unit_prices", skip_serializing_if = "Option::is_none")]
    pub unit_prices: Option<Vec<crate::models::ProjectsUsageTypeUnitPrice>>,
}

impl ProjectsUsageTypeUpdate {
    /// Used for update endpoint
    pub fn new() -> ProjectsUsageTypeUpdate {
        ProjectsUsageTypeUpdate {
            name: None,
            summary: None,
            category: None,
            service: None,
            unit_prices: None,
        }
    }
}



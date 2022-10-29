/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectsUsageTypeDb : Used for DB row entry



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectsUsageTypeDb {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "unit_prices")]
    pub unit_prices: Vec<crate::models::ProjectsUsageTypeUnitPrice>,
}

impl ProjectsUsageTypeDb {
    /// Used for DB row entry
    pub fn new(key: String, name: String, summary: String, category: String, service: String, unit_prices: Vec<crate::models::ProjectsUsageTypeUnitPrice>) -> ProjectsUsageTypeDb {
        ProjectsUsageTypeDb {
            key,
            name,
            summary,
            category,
            service,
            unit_prices,
        }
    }
}



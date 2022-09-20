/*
 * eHelply SDK - 1.1.109
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.109
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KpiResponse {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "service_uuid")]
    pub service_uuid: String,
    #[serde(rename = "kpis")]
    pub kpis: Vec<serde_json::Value>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "fetched_at", skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<String>,
}

impl KpiResponse {
    pub fn new(service_uuid: String, kpis: Vec<serde_json::Value>) -> KpiResponse {
        KpiResponse {
            uuid: None,
            service_uuid,
            kpis,
            created_at: None,
            fetched_at: None,
        }
    }
}



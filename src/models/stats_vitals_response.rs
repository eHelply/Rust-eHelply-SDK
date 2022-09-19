/*
 * eHelply SDK - 1.1.108
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.108
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatsVitalsResponse {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "service_uuid")]
    pub service_uuid: String,
    #[serde(rename = "heartbeat_uuid")]
    pub heartbeat_uuid: String,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<serde_json::Value>,
    #[serde(rename = "vitals", skip_serializing_if = "Option::is_none")]
    pub vitals: Option<serde_json::Value>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl StatsVitalsResponse {
    pub fn new(service_uuid: String, heartbeat_uuid: String) -> StatsVitalsResponse {
        StatsVitalsResponse {
            uuid: None,
            service_uuid,
            heartbeat_uuid,
            stats: None,
            vitals: None,
            created_at: None,
        }
    }
}



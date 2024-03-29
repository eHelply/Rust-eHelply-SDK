/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "service_uuid")]
    pub service_uuid: String,
    #[serde(rename = "stage")]
    pub stage: String,
    #[serde(rename = "process")]
    pub process: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "platform")]
    pub platform: serde_json::Value,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl HeartbeatResponse {
    pub fn new(service_uuid: String, stage: String, process: String, health: String, platform: serde_json::Value) -> HeartbeatResponse {
        HeartbeatResponse {
            uuid: None,
            service_uuid,
            stage,
            process,
            health,
            platform,
            created_at: None,
        }
    }
}



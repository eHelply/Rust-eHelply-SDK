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
pub struct AlarmResponse {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "service_uuid", skip_serializing_if = "Option::is_none")]
    pub service_uuid: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "stage", skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "ticket_uuid", skip_serializing_if = "Option::is_none")]
    pub ticket_uuid: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<serde_json::Value>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "latest_at", skip_serializing_if = "Option::is_none")]
    pub latest_at: Option<String>,
    #[serde(rename = "acknowledged_at", skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
    #[serde(rename = "ignored_at", skip_serializing_if = "Option::is_none")]
    pub ignored_at: Option<String>,
    #[serde(rename = "cleared_at", skip_serializing_if = "Option::is_none")]
    pub cleared_at: Option<String>,
    #[serde(rename = "terminated_at", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
    #[serde(rename = "assignee_uuid", skip_serializing_if = "Option::is_none")]
    pub assignee_uuid: Option<String>,
    #[serde(rename = "acknowledger_uuid", skip_serializing_if = "Option::is_none")]
    pub acknowledger_uuid: Option<String>,
    #[serde(rename = "ignorer_uuid", skip_serializing_if = "Option::is_none")]
    pub ignorer_uuid: Option<String>,
    #[serde(rename = "terminater_uuid", skip_serializing_if = "Option::is_none")]
    pub terminater_uuid: Option<String>,
}

impl AlarmResponse {
    pub fn new() -> AlarmResponse {
        AlarmResponse {
            uuid: None,
            service_uuid: None,
            count: None,
            stage: None,
            process: None,
            severity: None,
            ticket_uuid: None,
            name: None,
            summary: None,
            description: None,
            note: None,
            created_at: None,
            latest_at: None,
            acknowledged_at: None,
            ignored_at: None,
            cleared_at: None,
            terminated_at: None,
            assignee_uuid: None,
            acknowledger_uuid: None,
            ignorer_uuid: None,
            terminater_uuid: None,
        }
    }
}



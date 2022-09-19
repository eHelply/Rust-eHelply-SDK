/*
 * eHelply SDK - 1.1.115
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.115
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppointmentBase {
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
    #[serde(rename = "place_uuid", skip_serializing_if = "Option::is_none")]
    pub place_uuid: Option<String>,
    #[serde(rename = "review_group_uuid", skip_serializing_if = "Option::is_none")]
    pub review_group_uuid: Option<String>,
    #[serde(rename = "expected_finish_at", skip_serializing_if = "Option::is_none")]
    pub expected_finish_at: Option<String>,
    #[serde(rename = "expected_start_at", skip_serializing_if = "Option::is_none")]
    pub expected_start_at: Option<String>,
    #[serde(rename = "actual_start_at", skip_serializing_if = "Option::is_none")]
    pub actual_start_at: Option<String>,
    #[serde(rename = "actual_finish_at", skip_serializing_if = "Option::is_none")]
    pub actual_finish_at: Option<String>,
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<serde_json::Value>,
    #[serde(rename = "meta_uuid", skip_serializing_if = "Option::is_none")]
    pub meta_uuid: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "cancellation_at", skip_serializing_if = "Option::is_none")]
    pub cancellation_at: Option<String>,
    #[serde(rename = "cancellation_reason", skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "cancellation_entity_uuid", skip_serializing_if = "Option::is_none")]
    pub cancellation_entity_uuid: Option<String>,
}

impl AppointmentBase {
    pub fn new() -> AppointmentBase {
        AppointmentBase {
            project_uuid: None,
            place_uuid: None,
            review_group_uuid: None,
            expected_finish_at: None,
            expected_start_at: None,
            actual_start_at: None,
            actual_finish_at: None,
            products: None,
            meta_uuid: None,
            status: None,
            cancellation_at: None,
            cancellation_reason: None,
            cancellation_entity_uuid: None,
        }
    }
}



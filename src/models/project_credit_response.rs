/*
 * eHelply SDK - 1.1.114
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.114
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectCreditResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "credits_granted")]
    pub credits_granted: i32,
    #[serde(rename = "credits_consumed", skip_serializing_if = "Option::is_none")]
    pub credits_consumed: Option<i32>,
    #[serde(rename = "granted_by")]
    pub granted_by: String,
    #[serde(rename = "granted_reason")]
    pub granted_reason: String,
    #[serde(rename = "granted_at")]
    pub granted_at: String,
    #[serde(rename = "fully_consumed_at", skip_serializing_if = "Option::is_none")]
    pub fully_consumed_at: Option<String>,
    #[serde(rename = "revoked_reason", skip_serializing_if = "Option::is_none")]
    pub revoked_reason: Option<String>,
    #[serde(rename = "revoked_at", skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
}

impl ProjectCreditResponse {
    pub fn new(uuid: String, project_uuid: String, credits_granted: i32, granted_by: String, granted_reason: String, granted_at: String) -> ProjectCreditResponse {
        ProjectCreditResponse {
            uuid,
            project_uuid,
            credits_granted,
            credits_consumed: None,
            granted_by,
            granted_reason,
            granted_at,
            fully_consumed_at: None,
            revoked_reason: None,
            revoked_at: None,
        }
    }
}



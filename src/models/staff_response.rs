/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// StaffResponse : **:param** uuid                                **type:** string **:param** project_uuid                        **type:** string or None  **:param** entity                              **type:** string or None  **:param** place                               **type:** string or None  **:param** company                             **type:** string or None  **:param** schedule                            **type:** string or None  **:param** catalog                             **type:** string or None  **:param** reviews                             **type:** string or None  **:param** created_at                          **type:** string or None  **:param** updated_at                          **type:** string or None  **:param** deleted_at                          **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StaffResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<serde_json::Value>,
    #[serde(rename = "place", skip_serializing_if = "Option::is_none")]
    pub place: Option<serde_json::Value>,
    #[serde(rename = "place_roles", skip_serializing_if = "Option::is_none")]
    pub place_roles: Option<Vec<String>>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<serde_json::Value>,
    #[serde(rename = "company_roles", skip_serializing_if = "Option::is_none")]
    pub company_roles: Option<Vec<String>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<serde_json::Value>,
    #[serde(rename = "catalog", skip_serializing_if = "Option::is_none")]
    pub catalog: Option<serde_json::Value>,
    #[serde(rename = "reviews", skip_serializing_if = "Option::is_none")]
    pub reviews: Option<serde_json::Value>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl StaffResponse {
    /// **:param** uuid                                **type:** string **:param** project_uuid                        **type:** string or None  **:param** entity                              **type:** string or None  **:param** place                               **type:** string or None  **:param** company                             **type:** string or None  **:param** schedule                            **type:** string or None  **:param** catalog                             **type:** string or None  **:param** reviews                             **type:** string or None  **:param** created_at                          **type:** string or None  **:param** updated_at                          **type:** string or None  **:param** deleted_at                          **type:** string or None
    pub fn new(uuid: String) -> StaffResponse {
        StaffResponse {
            uuid,
            project_uuid: None,
            entity: None,
            place: None,
            place_roles: None,
            company: None,
            company_roles: None,
            schedule: None,
            catalog: None,
            reviews: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}



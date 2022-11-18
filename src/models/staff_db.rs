/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// StaffDb : **:param** uuid                                **type:** string **:param** project_uuid                        **type:** string or None  **:param** entity_uuid                         **type:** string or None  **:param** schedule_uuid                       **type:** string or None  **:param** catalog_uuid                        **type:** string or None  **:param** review_group_uuid                   **type:** string or None  **:param** created_at                          **type:** string or None  **:param** updated_at                          **type:** string or None  **:param** deleted_at                          **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StaffDb {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
    #[serde(rename = "entity_uuid", skip_serializing_if = "Option::is_none")]
    pub entity_uuid: Option<String>,
    #[serde(rename = "schedule_uuid", skip_serializing_if = "Option::is_none")]
    pub schedule_uuid: Option<String>,
    #[serde(rename = "catalog_uuid", skip_serializing_if = "Option::is_none")]
    pub catalog_uuid: Option<String>,
    #[serde(rename = "review_group_uuid", skip_serializing_if = "Option::is_none")]
    pub review_group_uuid: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl StaffDb {
    /// **:param** uuid                                **type:** string **:param** project_uuid                        **type:** string or None  **:param** entity_uuid                         **type:** string or None  **:param** schedule_uuid                       **type:** string or None  **:param** catalog_uuid                        **type:** string or None  **:param** review_group_uuid                   **type:** string or None  **:param** created_at                          **type:** string or None  **:param** updated_at                          **type:** string or None  **:param** deleted_at                          **type:** string or None
    pub fn new(uuid: String) -> StaffDb {
        StaffDb {
            uuid,
            project_uuid: None,
            entity_uuid: None,
            schedule_uuid: None,
            catalog_uuid: None,
            review_group_uuid: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}



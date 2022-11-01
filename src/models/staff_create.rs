/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// StaffCreate : **:param** project_uuid                        **type:** string or None  **:param** entity_uuid                         **type:** string or None  **:param** schedule_uuid                       **type:** string or None  **:param** catalog_uuid                        **type:** string or None  **:param** review_group_uuid                   **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StaffCreate {
    #[serde(rename = "entity_uuid")]
    pub entity_uuid: String,
    #[serde(rename = "project_uuid", skip_serializing_if = "Option::is_none")]
    pub project_uuid: Option<String>,
    #[serde(rename = "schedule_uuid", skip_serializing_if = "Option::is_none")]
    pub schedule_uuid: Option<String>,
    #[serde(rename = "catalog_uuid", skip_serializing_if = "Option::is_none")]
    pub catalog_uuid: Option<String>,
    #[serde(rename = "review_group_uuid", skip_serializing_if = "Option::is_none")]
    pub review_group_uuid: Option<String>,
}

impl StaffCreate {
    /// **:param** project_uuid                        **type:** string or None  **:param** entity_uuid                         **type:** string or None  **:param** schedule_uuid                       **type:** string or None  **:param** catalog_uuid                        **type:** string or None  **:param** review_group_uuid                   **type:** string or None
    pub fn new(entity_uuid: String) -> StaffCreate {
        StaffCreate {
            entity_uuid,
            project_uuid: None,
            schedule_uuid: None,
            catalog_uuid: None,
            review_group_uuid: None,
        }
    }
}



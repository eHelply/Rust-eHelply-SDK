/*
 * eHelply SDK - 1.1.112
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.112
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectsProjectMemberDb {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "entity_uuid")]
    pub entity_uuid: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl ProjectsProjectMemberDb {
    pub fn new(uuid: String, project_uuid: String, entity_uuid: String, role: String, created_at: String) -> ProjectsProjectMemberDb {
        ProjectsProjectMemberDb {
            uuid,
            project_uuid,
            entity_uuid,
            role,
            created_at,
        }
    }
}



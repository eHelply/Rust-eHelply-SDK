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
pub struct ProjectsProjectUsageDb {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "usage_key")]
    pub usage_key: String,
    #[serde(rename = "year")]
    pub year: i32,
    #[serde(rename = "month")]
    pub month: i32,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    #[serde(rename = "estimated_cost")]
    pub estimated_cost: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ProjectsProjectUsageDb {
    pub fn new(uuid: String, project_uuid: String, usage_key: String, year: i32, month: i32, quantity: i32, estimated_cost: i32, updated_at: String) -> ProjectsProjectUsageDb {
        ProjectsProjectUsageDb {
            uuid,
            project_uuid,
            usage_key,
            year,
            month,
            quantity,
            estimated_cost,
            updated_at,
        }
    }
}



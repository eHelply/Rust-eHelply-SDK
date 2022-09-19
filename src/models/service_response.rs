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
pub struct ServiceResponse {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "authors")]
    pub authors: Vec<String>,
    #[serde(rename = "author_emails")]
    pub author_emails: Vec<String>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "heartbeats", skip_serializing_if = "Option::is_none")]
    pub heartbeats: Option<Vec<serde_json::Value>>,
    #[serde(rename = "alarms", skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<crate::models::AlarmResponse>>,
    #[serde(rename = "hidden_stages", skip_serializing_if = "Option::is_none")]
    pub hidden_stages: Option<Vec<String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ServiceResponse {
    pub fn new(name: String, key: String, version: String, summary: String, authors: Vec<String>, author_emails: Vec<String>) -> ServiceResponse {
        ServiceResponse {
            name,
            key,
            version,
            summary,
            authors,
            author_emails,
            uuid: None,
            heartbeats: None,
            alarms: None,
            hidden_stages: None,
            created_at: None,
            updated_at: None,
        }
    }
}



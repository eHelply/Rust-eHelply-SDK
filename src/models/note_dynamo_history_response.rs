/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// NoteDynamoHistoryResponse : A note from Dynamo DB including n amount of version history of that note



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NoteDynamoHistoryResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::NoteMeta>,
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<crate::models::NoteDynamoResponse>>,
}

impl NoteDynamoHistoryResponse {
    /// A note from Dynamo DB including n amount of version history of that note
    pub fn new(uuid: String, content: String, time: String, meta: crate::models::NoteMeta) -> NoteDynamoHistoryResponse {
        NoteDynamoHistoryResponse {
            uuid,
            content,
            time,
            meta: Box::new(meta),
            history: None,
        }
    }
}



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
pub struct NoteDynamoResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::NoteMeta>,
}

impl NoteDynamoResponse {
    pub fn new(uuid: String, content: String, time: String, meta: crate::models::NoteMeta) -> NoteDynamoResponse {
        NoteDynamoResponse {
            uuid,
            content,
            time,
            meta: Box::new(meta),
        }
    }
}



/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// NoteBase : Note This is used as the payload to endpoints



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NoteBase {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::NoteMeta>,
}

impl NoteBase {
    /// Note This is used as the payload to endpoints
    pub fn new(content: String, time: String, meta: crate::models::NoteMeta) -> NoteBase {
        NoteBase {
            content,
            time,
            meta: Box::new(meta),
        }
    }
}



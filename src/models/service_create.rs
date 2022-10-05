/*
 * eHelply SDK - 1.1.111
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.111
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceCreate {
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
}

impl ServiceCreate {
    pub fn new(name: String, key: String, version: String, summary: String, authors: Vec<String>, author_emails: Vec<String>) -> ServiceCreate {
        ServiceCreate {
            name,
            key,
            version,
            summary,
            authors,
            author_emails,
        }
    }
}



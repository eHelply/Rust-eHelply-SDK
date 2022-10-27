/*
 * eHelply SDK - 1.1.114
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.114
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// Slugger : Meta slugger



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Slugger {
    #[serde(rename = "name")]
    pub name: String,
}

impl Slugger {
    /// Meta slugger
    pub fn new(name: String) -> Slugger {
        Slugger {
            name,
        }
    }
}


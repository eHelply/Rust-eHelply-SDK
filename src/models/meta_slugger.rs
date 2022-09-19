/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// MetaSlugger : Meta slugger



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaSlugger {
    #[serde(rename = "name")]
    pub name: String,
}

impl MetaSlugger {
    /// Meta slugger
    pub fn new(name: String) -> MetaSlugger {
        MetaSlugger {
            name,
        }
    }
}



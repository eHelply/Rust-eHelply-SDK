/*
 * eHelply SDK - 1.1.109
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.109
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Basic {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl Basic {
    pub fn new() -> Basic {
        Basic {
            name: None,
            slug: None,
        }
    }
}



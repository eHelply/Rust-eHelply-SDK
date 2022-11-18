/*
 * eHelply SDK - 1.1.122
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.122
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FactCreate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(rename = "public")]
    pub public: bool,
}

impl FactCreate {
    pub fn new(name: String, data: serde_json::Value, public: bool) -> FactCreate {
        FactCreate {
            name,
            data,
            public,
        }
    }
}



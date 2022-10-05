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
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<String>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl ValidationError {
    pub fn new(loc: Vec<String>, msg: String, _type: String) -> ValidationError {
        ValidationError {
            loc,
            msg,
            _type,
        }
    }
}



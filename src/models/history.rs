/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct History {
    #[serde(rename = "year")]
    pub year: i32,
    #[serde(rename = "month")]
    pub month: i32,
}

impl History {
    pub fn new(year: i32, month: i32) -> History {
        History {
            year,
            month,
        }
    }
}


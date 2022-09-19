/*
 * eHelply SDK - 1.1.108
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.108
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tax {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rate")]
    pub rate: i32,
}

impl Tax {
    pub fn new(name: String, rate: i32) -> Tax {
        Tax {
            name,
            rate,
        }
    }
}



/*
 * eHelply SDK - 1.1.113
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.113
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Discount {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "rate")]
    pub rate: i32,
    #[serde(rename = "flat")]
    pub flat: i32,
}

impl Discount {
    pub fn new(name: String, description: String, rate: i32, flat: i32) -> Discount {
        Discount {
            name,
            description,
            rate,
            flat,
        }
    }
}



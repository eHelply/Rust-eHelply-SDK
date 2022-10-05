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
pub struct CreateProjectCredit {
    #[serde(rename = "credits_granted")]
    pub credits_granted: i32,
    #[serde(rename = "granted_reason")]
    pub granted_reason: String,
}

impl CreateProjectCredit {
    pub fn new(credits_granted: i32, granted_reason: String) -> CreateProjectCredit {
        CreateProjectCredit {
            credits_granted,
            granted_reason,
        }
    }
}



/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// UserSignupReturn : Default participant UUID



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserSignupReturn {
    #[serde(rename = "participant_uuid")]
    pub participant_uuid: String,
}

impl UserSignupReturn {
    /// Default participant UUID
    pub fn new(participant_uuid: String) -> UserSignupReturn {
        UserSignupReturn {
            participant_uuid,
        }
    }
}



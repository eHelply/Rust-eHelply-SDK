/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateReview {
    #[serde(rename = "rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(rename = "max_rating", skip_serializing_if = "Option::is_none")]
    pub max_rating: Option<i32>,
    #[serde(rename = "review_text", skip_serializing_if = "Option::is_none")]
    pub review_text: Option<String>,
}

impl UpdateReview {
    pub fn new() -> UpdateReview {
        UpdateReview {
            rating: None,
            max_rating: None,
            review_text: None,
        }
    }
}



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
pub struct CreateReview {
    #[serde(rename = "rating")]
    pub rating: i32,
    #[serde(rename = "max_rating")]
    pub max_rating: i32,
    #[serde(rename = "review_text")]
    pub review_text: String,
}

impl CreateReview {
    pub fn new(rating: i32, max_rating: i32, review_text: String) -> CreateReview {
        CreateReview {
            rating,
            max_rating,
            review_text,
        }
    }
}



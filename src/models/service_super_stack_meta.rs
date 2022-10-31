/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceSuperStackMeta {
    #[serde(rename = "human_name")]
    pub human_name: String,
    #[serde(rename = "route_name")]
    pub route_name: String,
    #[serde(rename = "service_name")]
    pub service_name: String,
    #[serde(rename = "advertise")]
    pub advertise: bool,
    #[serde(rename = "featured")]
    pub featured: bool,
    #[serde(rename = "picture")]
    pub picture: String,
    #[serde(rename = "background_picture")]
    pub background_picture: String,
    #[serde(rename = "tag_line")]
    pub tag_line: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "features")]
    pub features: Vec<crate::models::ServiceSuperStackMetaFeature>,
    #[serde(rename = "use_cases")]
    pub use_cases: Vec<crate::models::ServiceSuperStackMetaUseCase>,
    #[serde(rename = "getting_started")]
    pub getting_started: Box<crate::models::ServiceSuperStackMetaGettingStarted>,
    #[serde(rename = "faqs")]
    pub faqs: Vec<crate::models::ServiceSuperStackMetaFaq>,
}

impl ServiceSuperStackMeta {
    pub fn new(human_name: String, route_name: String, service_name: String, advertise: bool, featured: bool, picture: String, background_picture: String, tag_line: String, summary: String, description: String, features: Vec<crate::models::ServiceSuperStackMetaFeature>, use_cases: Vec<crate::models::ServiceSuperStackMetaUseCase>, getting_started: crate::models::ServiceSuperStackMetaGettingStarted, faqs: Vec<crate::models::ServiceSuperStackMetaFaq>) -> ServiceSuperStackMeta {
        ServiceSuperStackMeta {
            human_name,
            route_name,
            service_name,
            advertise,
            featured,
            picture,
            background_picture,
            tag_line,
            summary,
            description,
            features,
            use_cases,
            getting_started: Box::new(getting_started),
            faqs,
        }
    }
}



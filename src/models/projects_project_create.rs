/*
 * eHelply SDK - 1.1.117
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.117
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectsProjectCreate : Used for create endpoint



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectsProjectCreate {
    #[serde(rename = "name")]
    pub name: String,
}

impl ProjectsProjectCreate {
    /// Used for create endpoint
    pub fn new(name: String) -> ProjectsProjectCreate {
        ProjectsProjectCreate {
            name,
        }
    }
}



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
pub struct GetProjectInvoiceHistory {
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    #[serde(rename = "invoice_history", skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<Vec<crate::models::History>>,
}

impl GetProjectInvoiceHistory {
    pub fn new(project_uuid: String) -> GetProjectInvoiceHistory {
        GetProjectInvoiceHistory {
            project_uuid,
            invoice_history: None,
        }
    }
}



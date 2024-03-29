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
pub struct GetInvoiceResponse {
    #[serde(rename = "invoice_uuid")]
    pub invoice_uuid: String,
    #[serde(rename = "line_items")]
    pub line_items: Vec<crate::models::LineItem>,
    #[serde(rename = "subtotal")]
    pub subtotal: i32,
    #[serde(rename = "discounts")]
    pub discounts: Vec<crate::models::Discount>,
    #[serde(rename = "taxes")]
    pub taxes: Vec<crate::models::Tax>,
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "notes")]
    pub notes: Vec<crate::models::Note>,
    #[serde(rename = "paid", skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "update_at")]
    pub update_at: String,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl GetInvoiceResponse {
    pub fn new(invoice_uuid: String, line_items: Vec<crate::models::LineItem>, subtotal: i32, discounts: Vec<crate::models::Discount>, taxes: Vec<crate::models::Tax>, total: i32, notes: Vec<crate::models::Note>, created_at: String, update_at: String) -> GetInvoiceResponse {
        GetInvoiceResponse {
            invoice_uuid,
            line_items,
            subtotal,
            discounts,
            taxes,
            total,
            notes,
            paid: None,
            created_at,
            update_at,
            deleted_at: None,
        }
    }
}



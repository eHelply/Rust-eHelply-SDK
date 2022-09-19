/*
 * eHelply SDK - 1.1.107
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.107
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTransactionResponse {
    #[serde(rename = "invoice", skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<crate::models::GetInvoiceResponse>>,
    #[serde(rename = "transaction_uuid")]
    pub transaction_uuid: String,
    #[serde(rename = "stripe_id")]
    pub stripe_id: String,
    #[serde(rename = "credit")]
    pub credit: i32,
    #[serde(rename = "debit")]
    pub debit: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl GetTransactionResponse {
    pub fn new(transaction_uuid: String, stripe_id: String, credit: i32, debit: i32, created_at: String) -> GetTransactionResponse {
        GetTransactionResponse {
            invoice: None,
            transaction_uuid,
            stripe_id,
            credit,
            debit,
            created_at,
        }
    }
}


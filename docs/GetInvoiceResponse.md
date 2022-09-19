# GetInvoiceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_uuid** | **String** |  | 
**line_items** | [**Vec<crate::models::LineItem>**](LineItem.md) |  | 
**subtotal** | **i32** |  | 
**discounts** | [**Vec<crate::models::Discount>**](Discount.md) |  | 
**taxes** | [**Vec<crate::models::Tax>**](Tax.md) |  | 
**total** | **i32** |  | 
**notes** | [**Vec<crate::models::Note>**](Note.md) |  | 
**paid** | Option<**bool**> |  | [optional][default to false]
**created_at** | **String** |  | 
**update_at** | **String** |  | 
**deleted_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



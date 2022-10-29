# ServiceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**key** | **String** |  | 
**version** | **String** |  | 
**summary** | **String** |  | 
**authors** | **Vec<String>** |  | 
**author_emails** | **Vec<String>** |  | 
**uuid** | Option<**String**> |  | [optional]
**heartbeats** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**alarms** | Option<[**Vec<crate::models::AlarmResponse>**](AlarmResponse.md)> |  | [optional]
**hidden_stages** | Option<**Vec<String>**> |  | [optional][default to []]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**superstack_meta** | Option<[**crate::models::ServiceSuperStackMeta**](ServiceSuperStackMeta.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



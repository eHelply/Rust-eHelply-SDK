# \LoggingApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logs**](LoggingApi.md#get_logs) | **GET** /sam/logging/logs | Getlogs
[**get_service_logs**](LoggingApi.md#get_service_logs) | **GET** /sam/logging/logs/services/{service} | Getservicelogs
[**get_subject_logs**](LoggingApi.md#get_subject_logs) | **GET** /sam/logging/logs/services/{service}/subjects/{subject} | Getsubjectlogs



## get_logs

> Vec<crate::models::LoggingDynamo> get_logs(service, date_start, date_end, desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getlogs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<**String**> |  |  |
**date_start** | Option<**String**> |  |  |
**date_end** | Option<**String**> |  |  |
**desc** | Option<**bool**> |  |  |[default to true]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::LoggingDynamo>**](LoggingDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_logs

> Vec<crate::models::LoggingDynamo> get_service_logs(service, date_start, date_end, desc, limit, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getservicelogs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**date_start** | Option<**String**> |  |  |
**date_end** | Option<**String**> |  |  |
**desc** | Option<**bool**> |  |  |[default to true]
**limit** | Option<**i32**> |  |  |[default to 50]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::LoggingDynamo>**](LoggingDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subject_logs

> serde_json::Value get_subject_logs(service, subject, date_start, date_end, desc, limit, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getsubjectlogs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**subject** | **String** |  | [required] |
**date_start** | Option<**String**> |  |  |
**date_end** | Option<**String**> |  |  |
**desc** | Option<**bool**> |  |  |[default to true]
**limit** | Option<**i32**> |  |  |[default to 50]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \BillingApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_billing_account**](BillingApi.md#create_billing_account) | **POST** /sam/billing/create_billing_account | Createbillingaccount
[**get_client_secret**](BillingApi.md#get_client_secret) | **GET** /sam/billing/retrieve_secret | Getclientsecret
[**has_payment**](BillingApi.md#has_payment) | **GET** /sam/billing/has_payment | Haspayment
[**list_payment_methods**](BillingApi.md#list_payment_methods) | **GET** /sam/billing/view_payment_method | Listpaymentmethods
[**process_payment**](BillingApi.md#process_payment) | **POST** /sam/billing/process_payment | Processpayment
[**reconcile_payment_method**](BillingApi.md#reconcile_payment_method) | **GET** /sam/billing/reconcile_payment | Reconcilepaymentmethod
[**remove_payment_method**](BillingApi.md#remove_payment_method) | **DELETE** /sam/billing/remove_payment_method | Removepaymentmethod



## create_billing_account

> crate::models::StripeAccountResponse create_billing_account(x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createbillingaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::StripeAccountResponse**](StripeAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_secret

> crate::models::StripeCustomerSecretResponse get_client_secret(x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getclientsecret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::StripeCustomerSecretResponse**](StripeCustomerSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_payment

> bool has_payment(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Haspayment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<[**serde_json::Value**](.md)> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_methods

> Vec<crate::models::PaymentMethodResponse> list_payment_methods(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Listpaymentmethods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<[**serde_json::Value**](.md)> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PaymentMethodResponse>**](PaymentMethodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_payment

> String process_payment(payment, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Processpayment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment** | [**Payment**](Payment.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reconcile_payment_method

> bool reconcile_payment_method(x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Reconcilepaymentmethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_payment_method

> String remove_payment_method(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Removepaymentmethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<[**serde_json::Value**](.md)> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


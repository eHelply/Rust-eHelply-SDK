# \SecurityApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_encryption_key**](SecurityApi.md#create_encryption_key) | **POST** /sam/security/encryption/categories/{category}/keys | Createencryptionkey
[**create_key**](SecurityApi.md#create_key) | **POST** /sam/security/keys | Createkey
[**delete_key**](SecurityApi.md#delete_key) | **DELETE** /sam/security/keys/{key_uuid} | Deletekey
[**generate_token**](SecurityApi.md#generate_token) | **POST** /sam/security/tokens | Generatetoken
[**get_encryption_key**](SecurityApi.md#get_encryption_key) | **GET** /sam/security/encryption/categories/{category}/keys | Getencryptionkey
[**get_key**](SecurityApi.md#get_key) | **GET** /sam/security/keys/{key_uuid} | Getkey
[**search_keys**](SecurityApi.md#search_keys) | **GET** /sam/security/keys | Searchkeys
[**verify_key**](SecurityApi.md#verify_key) | **POST** /sam/security/keys/verify | Verifykey



## create_encryption_key

> crate::models::SecurityEncryptionKeyResponse create_encryption_key(category, ehelply_security_secret_key)
Createencryptionkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**ehelply_security_secret_key** | Option<**String**> |  |  |

### Return type

[**crate::models::SecurityEncryptionKeyResponse**](SecurityEncryptionKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_key

> crate::models::ResponseCreatekey create_key(security_key_create, access_length, secret_length)
Createkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_key_create** | [**SecurityKeyCreate**](SecurityKeyCreate.md) |  | [required] |
**access_length** | Option<**i32**> |  |  |[default to 64]
**secret_length** | Option<**i32**> |  |  |[default to 64]

### Return type

[**crate::models::ResponseCreatekey**](Response_Createkey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_key

> crate::models::ResponseDeletekey delete_key(key_uuid)
Deletekey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_uuid** | **String** |  | [required] |

### Return type

[**crate::models::ResponseDeletekey**](Response_Deletekey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_token

> crate::models::ResponseGeneratetoken generate_token(security_create_token)
Generatetoken

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_create_token** | [**SecurityCreateToken**](SecurityCreateToken.md) |  | [required] |

### Return type

[**crate::models::ResponseGeneratetoken**](Response_Generatetoken.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_encryption_key

> Vec<crate::models::SecurityEncryptionKeyGet> get_encryption_key(category, ehelply_security_secret_key)
Getencryptionkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**ehelply_security_secret_key** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SecurityEncryptionKeyGet>**](SecurityEncryptionKeyGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_key

> crate::models::SecurityKeyGet get_key(key_uuid)
Getkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_uuid** | **String** |  | [required] |

### Return type

[**crate::models::SecurityKeyGet**](SecurityKeyGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_keys

> Vec<crate::models::SecurityKeyGet> search_keys()
Searchkeys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SecurityKeyGet>**](SecurityKeyGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_key

> crate::models::SecurityKeyGet verify_key(security_key_verify)
Verifykey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_key_verify** | [**SecurityKeyVerify**](SecurityKeyVerify.md) |  | [required] |

### Return type

[**crate::models::SecurityKeyGet**](SecurityKeyGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


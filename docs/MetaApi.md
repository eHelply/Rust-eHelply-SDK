# \MetaApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_field**](MetaApi.md#create_field) | **POST** /meta/field | Create Field
[**create_meta**](MetaApi.md#create_meta) | **POST** /meta/meta/service/{service}/type/{type_str}/entity/{entity_uuid} | Create Meta
[**delete_field**](MetaApi.md#delete_field) | **DELETE** /meta/field/{field_uuid} | Delete Field
[**delete_meta**](MetaApi.md#delete_meta) | **DELETE** /meta/meta/service/{service}/type/{type_str}/entity/{entity_uuid} | Delete Meta
[**delete_meta_from_uuid**](MetaApi.md#delete_meta_from_uuid) | **DELETE** /meta/meta/{meta_uuid} | Delete Meta From Uuid
[**get_field**](MetaApi.md#get_field) | **GET** /meta/field/{field_uuid} | Get Field
[**get_meta**](MetaApi.md#get_meta) | **GET** /meta/meta/service/{service}/type/{type_str}/entity/{entity_uuid} | Get Meta
[**get_meta_from_uuid**](MetaApi.md#get_meta_from_uuid) | **GET** /meta/meta/{meta_uuid} | Get Meta From Uuid
[**make_slug**](MetaApi.md#make_slug) | **POST** /meta/meta/slug | Make Slug
[**touch_meta**](MetaApi.md#touch_meta) | **POST** /meta/meta/service/{service}/type/{type_str}/entity/{entity_uuid}/touch | Touch Meta
[**update_field**](MetaApi.md#update_field) | **PUT** /meta/field/{field_uuid} | Update Field
[**update_meta**](MetaApi.md#update_meta) | **PUT** /meta/meta/service/{service}/type/{type_str}/entity/{entity_uuid} | Update Meta
[**update_meta_from_uuid**](MetaApi.md#update_meta_from_uuid) | **PUT** /meta/meta/{meta_uuid} | Update Meta From Uuid



## create_field

> crate::models::FieldDynamo create_field(field, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Create Field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | [**Field**](Field.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::FieldDynamo**](FieldDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_meta

> crate::models::MetaDynamo create_meta(service, type_str, entity_uuid, meta_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Create Meta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**type_str** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
**meta_create** | [**MetaCreate**](MetaCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::MetaDynamo**](MetaDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_field

> serde_json::Value delete_field(field_uuid, soft_delete, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Delete Field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_uuid** | **String** |  | [required] |
**soft_delete** | Option<**bool**> |  |  |[default to true]
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


## delete_meta

> serde_json::Value delete_meta(service, type_str, entity_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Delete Meta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**type_str** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
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


## delete_meta_from_uuid

> serde_json::Value delete_meta_from_uuid(meta_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Delete Meta From Uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta_uuid** | **String** |  | [required] |
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


## get_field

> crate::models::FieldDynamo get_field(field_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Get Field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::FieldDynamo**](FieldDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_meta

> crate::models::MetaGet get_meta(service, type_str, entity_uuid, detailed, custom, dates, history, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Get Meta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**type_str** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
**detailed** | Option<**bool**> |  |  |[default to false]
**custom** | Option<**bool**> |  |  |[default to false]
**dates** | Option<**bool**> |  |  |[default to false]
**history** | Option<**i32**> |  |  |[default to 0]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::MetaGet**](MetaGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_meta_from_uuid

> crate::models::MetaGet get_meta_from_uuid(meta_uuid, detailed, custom, dates, history, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Get Meta From Uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta_uuid** | **String** |  | [required] |
**detailed** | Option<**bool**> |  |  |[default to false]
**custom** | Option<**bool**> |  |  |[default to false]
**dates** | Option<**bool**> |  |  |[default to false]
**history** | Option<**i32**> |  |  |[default to 0]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::MetaGet**](MetaGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## make_slug

> serde_json::Value make_slug(meta_slugger)
Make Slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta_slugger** | [**MetaSlugger**](MetaSlugger.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## touch_meta

> crate::models::MetaDynamo touch_meta(service, type_str, entity_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Touch Meta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**type_str** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::MetaDynamo**](MetaDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field

> serde_json::Value update_field(field_uuid, field, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Update Field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_uuid** | **String** |  | [required] |
**field** | [**Field**](Field.md) |  | [required] |
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

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_meta

> crate::models::MetaDynamo update_meta(service, type_str, entity_uuid, meta_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Update Meta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**type_str** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
**meta_create** | [**MetaCreate**](MetaCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::MetaDynamo**](MetaDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_meta_from_uuid

> crate::models::MetaDynamo update_meta_from_uuid(meta_uuid, meta_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Update Meta From Uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta_uuid** | **String** |  | [required] |
**meta_create** | [**MetaCreate**](MetaCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::MetaDynamo**](MetaDynamo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


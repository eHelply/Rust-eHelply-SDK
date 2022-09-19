# \CategoryApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_category_places_categories_post**](CategoryApi.md#create_category_places_categories_post) | **POST** /places/categories | Create Category
[**delete_category_places_categories_category_uuid_delete**](CategoryApi.md#delete_category_places_categories_category_uuid_delete) | **DELETE** /places/categories/{category_uuid} | Delete Category
[**get_category_places_categories_category_uuid_get**](CategoryApi.md#get_category_places_categories_category_uuid_get) | **GET** /places/categories/{category_uuid} | Get Category
[**search_categories_places_categories_get**](CategoryApi.md#search_categories_places_categories_get) | **GET** /places/categories | Search Categories
[**update_category_places_categories_category_uuid_put**](CategoryApi.md#update_category_places_categories_category_uuid_put) | **PUT** /places/categories/{category_uuid} | Update Category



## create_category_places_categories_post

> crate::models::CategoryResponse create_category_places_categories_post(category_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Create Category

Creates a category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_base** | [**CategoryBase**](CategoryBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CategoryResponse**](CategoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_category_places_categories_category_uuid_delete

> serde_json::Value delete_category_places_categories_category_uuid_delete(category_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Delete Category

Deletes the category with the given ID and returns True if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_uuid** | **String** |  | [required] |
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


## get_category_places_categories_category_uuid_get

> crate::models::CategoryResponse get_category_places_categories_category_uuid_get(category_uuid, with_meta, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Get Category

Gets the category information given the category ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_uuid** | **String** |  | [required] |
**with_meta** | Option<**bool**> |  |  |[default to false]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CategoryResponse**](CategoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_categories_places_categories_get

> crate::models::Page search_categories_places_categories_get(project_uuid, name, with_meta, page, page_size, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Search Categories

TODO Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      name                                **type:** string or None      meta                                **type:** dict or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**with_meta** | Option<**bool**> |  |  |[default to false]
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 25]
**sort_on** | Option<**String**> |  |  |
**sort_desc** | Option<**bool**> |  |  |[default to false]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_category_places_categories_category_uuid_put

> crate::models::CategoryBase update_category_places_categories_category_uuid_put(category_uuid, category_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Update Category

Update category with given info, only updating the fields supplied. Category Uuid must be sent however.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_uuid** | **String** |  | [required] |
**category_base** | [**CategoryBase**](CategoryBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CategoryBase**](CategoryBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \TagsApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](TagsApi.md#create_tag) | **POST** /places/tags | Createtag
[**get_tag**](TagsApi.md#get_tag) | **GET** /places/tags/{tag_uuid} | Gettag
[**search_tag**](TagsApi.md#search_tag) | **GET** /places/tags | Searchtag
[**update_tag**](TagsApi.md#update_tag) | **PUT** /places/tags/{tag_uuid} | Updatetag



## create_tag

> crate::models::TagResponse create_tag(tag_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createtag

Creates a tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_base** | [**TagBase**](TagBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::TagResponse**](TagResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> crate::models::TagResponse get_tag(tag_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Gettag

Gets the tag member information given the tag ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::TagResponse**](TagResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_tag

> crate::models::Page search_tag(project_uuid, name, page, page_size, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Searchtag

TODO Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      name                                **type:** string or None      meta                                **type:** dict or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
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


## update_tag

> crate::models::TagResponse update_tag(tag_uuid, tag_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updatetag

Update tag with given info, only updating the fields supplied. Tag Uuid must be sent however.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_uuid** | **String** |  | [required] |
**tag_base** | [**TagBase**](TagBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::TagResponse**](TagResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


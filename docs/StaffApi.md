# \StaffApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_staff**](StaffApi.md#create_staff) | **POST** /places/staff | Createstaff
[**delete_staff**](StaffApi.md#delete_staff) | **DELETE** /places/staff/{staff_uuid} | Deletestaff
[**get_staff**](StaffApi.md#get_staff) | **GET** /places/staff/{staff_uuid} | Getstaff
[**search_staff**](StaffApi.md#search_staff) | **GET** /places/staff | Searchstaff
[**update_staff**](StaffApi.md#update_staff) | **PUT** /places/staff/{staff_uuid} | Updatestaff



## create_staff

> crate::models::StaffDb create_staff(staff_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createstaff

Creates a staff member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**staff_create** | [**StaffCreate**](StaffCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::StaffDb**](StaffDb.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_staff

> serde_json::Value delete_staff(staff_uuid, soft_delete, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deletestaff

Deletes the staff member with the given ID and returns True if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**staff_uuid** | **String** |  | [required] |
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


## get_staff

> crate::models::StaffResponse get_staff(staff_uuid, with_places, with_companies, with_catalog, with_schedule, with_roles, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getstaff

Gets the staff member information given the staff ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**staff_uuid** | **String** |  | [required] |
**with_places** | Option<**bool**> |  |  |[default to false]
**with_companies** | Option<**bool**> |  |  |[default to false]
**with_catalog** | Option<**bool**> |  |  |[default to false]
**with_schedule** | Option<**bool**> |  |  |[default to false]
**with_roles** | Option<**bool**> |  |  |[default to false]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::StaffResponse**](StaffResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_staff

> crate::models::Page search_staff(project_uuid, first_name, last_name, is_deleted, with_companies, with_places, with_schedule, with_catalog, with_reviews, with_roles, page, page_size, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Searchstaff

TODO Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      entity                              **type:** string or None      place                               **type:** dict or None      company                             **type:** dict or None      schedule                            **type:** dict or None      catalog                             **type:** dict or None      reviews                             **type:** dict or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<**String**> |  |  |
**first_name** | Option<**String**> |  |  |
**last_name** | Option<**String**> |  |  |
**is_deleted** | Option<**bool**> |  |  |[default to false]
**with_companies** | Option<**bool**> |  |  |[default to false]
**with_places** | Option<**bool**> |  |  |[default to false]
**with_schedule** | Option<**bool**> |  |  |[default to false]
**with_catalog** | Option<**bool**> |  |  |[default to false]
**with_reviews** | Option<**bool**> |  |  |[default to false]
**with_roles** | Option<**bool**> |  |  |[default to false]
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


## update_staff

> crate::models::StaffResponse update_staff(staff_uuid, staff_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updatestaff

Update staff with given info, only updating the fields supplied. Staff Uuid must be sent however.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**staff_uuid** | **String** |  | [required] |
**staff_create** | [**StaffCreate**](StaffCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::StaffResponse**](StaffResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \CompaniesApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_company_places_companies_post**](CompaniesApi.md#create_company_places_companies_post) | **POST** /places/companies | Create Company
[**delete_company_places_companies_company_uuid_delete**](CompaniesApi.md#delete_company_places_companies_company_uuid_delete) | **DELETE** /places/companies/{company_uuid} | Delete Company
[**get_company_places_companies_company_uuid_get**](CompaniesApi.md#get_company_places_companies_company_uuid_get) | **GET** /places/companies/{company_uuid} | Get Company
[**search_companies_places_companies_get**](CompaniesApi.md#search_companies_places_companies_get) | **GET** /places/companies | Search Companies
[**update_company_places_companies_company_uuid_put**](CompaniesApi.md#update_company_places_companies_company_uuid_put) | **PUT** /places/companies/{company_uuid} | Update Company



## create_company_places_companies_post

> crate::models::CompanyResponse create_company_places_companies_post(company_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Create Company

Creates a company

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_base** | [**CompanyBase**](CompanyBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CompanyResponse**](CompanyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_company_places_companies_company_uuid_delete

> serde_json::Value delete_company_places_companies_company_uuid_delete(company_uuid, soft_delete, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Delete Company

Deletes the company with the given ID and returns True if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_uuid** | **String** |  | [required] |
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


## get_company_places_companies_company_uuid_get

> crate::models::CompanyResponse get_company_places_companies_company_uuid_get(company_uuid, with_meta, with_catalog, with_reviews, with_schedule, with_blog, with_tags, with_categories, with_places, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Get Company

Gets the company information given the Place ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_uuid** | **String** |  | [required] |
**with_meta** | Option<**bool**> |  |  |[default to false]
**with_catalog** | Option<**bool**> |  |  |[default to false]
**with_reviews** | Option<**bool**> |  |  |[default to false]
**with_schedule** | Option<**bool**> |  |  |[default to false]
**with_blog** | Option<**bool**> |  |  |[default to false]
**with_tags** | Option<**bool**> |  |  |[default to false]
**with_categories** | Option<**bool**> |  |  |[default to false]
**with_places** | Option<**bool**> |  |  |[default to false]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CompanyResponse**](CompanyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_companies_places_companies_get

> crate::models::Page search_companies_places_companies_get(project_uuid, name, email, is_public, is_deleted, with_places, with_meta, with_catalog, with_reviews, with_schedule, with_blog, with_tags, with_categories, page, page_size, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Search Companies

Search all companies and returns paginated results with Companies being stored in items field. Can search by `project_uuid, name, email` string fields or the `is_public and is_deleted` boolean fields. To search with these fields use query params with string values. For sorting fill out \"sort_desc\" field with either true/false and the \"sort_on\" query parameter with column you want to sort on (ex: name). Max pagination items per page is 50. Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      meta_uuid                           **type:** string or None      catalog_data                        **type:** dict or None      review_group_data                   **type:** dict or None      schedule_data                       **type:** dict or None      blog_data                           **type:** dict or None      tags                                **type:** [TagBase] or None      categories                          **type:** [CategoryBase] or None      places                              **type:** PlaceBase or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |
**is_public** | Option<**bool**> |  |  |[default to true]
**is_deleted** | Option<**bool**> |  |  |[default to false]
**with_places** | Option<**bool**> |  |  |[default to false]
**with_meta** | Option<**bool**> |  |  |[default to false]
**with_catalog** | Option<**bool**> |  |  |[default to false]
**with_reviews** | Option<**bool**> |  |  |[default to false]
**with_schedule** | Option<**bool**> |  |  |[default to false]
**with_blog** | Option<**bool**> |  |  |[default to false]
**with_tags** | Option<**bool**> |  |  |[default to false]
**with_categories** | Option<**bool**> |  |  |[default to false]
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


## update_company_places_companies_company_uuid_put

> crate::models::CompanyResponse update_company_places_companies_company_uuid_put(company_uuid, company_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Update Company

Update company with given info, only updating the fields supplied. Company Uuid must be sent however.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_uuid** | **String** |  | [required] |
**company_base** | [**CompanyBase**](CompanyBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CompanyResponse**](CompanyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


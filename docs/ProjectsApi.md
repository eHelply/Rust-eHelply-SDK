# \ProjectsApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_member_to_project**](ProjectsApi.md#add_member_to_project) | **POST** /sam/projects/projects/{project_uuid}/members/{entity_uuid} | Addmembertoproject
[**archive_project**](ProjectsApi.md#archive_project) | **DELETE** /sam/projects/projects/{project_uuid} | Archiveproject
[**create_project**](ProjectsApi.md#create_project) | **POST** /sam/projects/projects | Createproject
[**create_project_credential**](ProjectsApi.md#create_project_credential) | **POST** /sam/projects/projects/{project_uuid}/credentials | Createprojectcredential
[**create_project_credit**](ProjectsApi.md#create_project_credit) | **POST** /sam/projects/projects/{project_uuid}/credits | Createprojectcredit
[**create_project_invoice**](ProjectsApi.md#create_project_invoice) | **POST** /sam/projects/projects/{project_uuid}/invoices | Createprojectinvoice
[**create_project_key**](ProjectsApi.md#create_project_key) | **POST** /sam/projects/projects/{project_uuid}/keys | Createprojectkey
[**create_usage_type**](ProjectsApi.md#create_usage_type) | **POST** /sam/projects/usage/types | Createusagetype
[**delete_project_credential**](ProjectsApi.md#delete_project_credential) | **DELETE** /sam/projects/projects/{project_uuid}/credentials/{service_name} | Deleteprojectcredential
[**delete_project_key**](ProjectsApi.md#delete_project_key) | **DELETE** /sam/projects/projects/{project_uuid}/keys | Deleteprojectkey
[**delete_usage_type**](ProjectsApi.md#delete_usage_type) | **DELETE** /sam/projects/usage/types/{usage_type_key} | Deleteusagetype
[**get_all_project_credentials**](ProjectsApi.md#get_all_project_credentials) | **GET** /sam/projects/projects/{project_uuid}/credentials | Getallprojectcredentials
[**get_all_project_credits**](ProjectsApi.md#get_all_project_credits) | **GET** /sam/projects/projects/{project_uuid}/credits | Getallprojectcredits
[**get_all_project_usage**](ProjectsApi.md#get_all_project_usage) | **GET** /sam/projects/projects/{project_uuid}/usage | Getallprojectusage
[**get_member_projects**](ProjectsApi.md#get_member_projects) | **GET** /sam/projects/members/{entity_uuid}/projects | Getmemberprojects
[**get_project**](ProjectsApi.md#get_project) | **GET** /sam/projects/projects/{project_uuid} | Getproject
[**get_project_credit_transactions**](ProjectsApi.md#get_project_credit_transactions) | **GET** /sam/projects/projects/{project_uuid}/credits/{credit_uuid}/transactions | Getprojectcredittransactions
[**get_project_invoice**](ProjectsApi.md#get_project_invoice) | **GET** /sam/projects/projects/{project_uuid}/invoices | Getprojectinvoice
[**get_project_invoice_history**](ProjectsApi.md#get_project_invoice_history) | **GET** /sam/projects/projects/{project_uuid}/invoices/history | Getprojectinvoicehistory
[**get_project_keys**](ProjectsApi.md#get_project_keys) | **GET** /sam/projects/projects/{project_uuid}/keys | Getprojectkeys
[**get_project_members**](ProjectsApi.md#get_project_members) | **GET** /sam/projects/projects/{project_uuid}/members | Getprojectmembers
[**get_specific_project_credential**](ProjectsApi.md#get_specific_project_credential) | **GET** /sam/projects/projects/{project_uuid}/credentials/{service_name} | Getspecificprojectcredential
[**get_specific_project_usage**](ProjectsApi.md#get_specific_project_usage) | **GET** /sam/projects/projects/{project_uuid}/usage/{usage_type_key} | Getspecificprojectusage
[**get_usage_type**](ProjectsApi.md#get_usage_type) | **GET** /sam/projects/usage/types/{usage_type_key} | Getusagetype
[**remove_member_from_project**](ProjectsApi.md#remove_member_from_project) | **DELETE** /sam/projects/projects/{project_uuid}/members/{entity_uuid} | Removememberfromproject
[**revoke_project_credit**](ProjectsApi.md#revoke_project_credit) | **DELETE** /sam/projects/projects/{project_uuid}/credits/{credit_uuid} | Revokeprojectcredit
[**search_projects**](ProjectsApi.md#search_projects) | **GET** /sam/projects/projects | Searchprojects
[**search_usage_type**](ProjectsApi.md#search_usage_type) | **GET** /sam/projects/usage/types | Searchusagetype
[**update_project**](ProjectsApi.md#update_project) | **PUT** /sam/projects/projects/{project_uuid} | Updateproject
[**update_project_credential**](ProjectsApi.md#update_project_credential) | **PUT** /sam/projects/projects/{project_uuid}/credentials/{service_name} | Updateprojectcredential
[**update_usage_type**](ProjectsApi.md#update_usage_type) | **PUT** /sam/projects/usage/types/{usage_type_key} | Updateusagetype



## add_member_to_project

> crate::models::ResponseAddmembertoproject add_member_to_project(project_uuid, entity_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Addmembertoproject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseAddmembertoproject**](Response_Addmembertoproject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_project

> crate::models::ResponseArchiveproject archive_project(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Archiveproject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseArchiveproject**](Response_Archiveproject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project

> crate::models::ProjectDb create_project(projects_project_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createproject

Create a new Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_project_create** | [**ProjectsProjectCreate**](ProjectsProjectCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectDb**](ProjectDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_credential

> crate::models::ResponseCreateprojectcredential create_project_credential(project_uuid, create_project_credential, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createprojectcredential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**create_project_credential** | [**CreateProjectCredential**](CreateProjectCredential.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseCreateprojectcredential**](Response_Createprojectcredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_credit

> crate::models::ProjectCreditResponse create_project_credit(project_uuid, create_project_credit, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createprojectcredit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**create_project_credit** | [**CreateProjectCredit**](CreateProjectCredit.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectCreditResponse**](ProjectCreditResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_invoice

> crate::models::ResponseCreateprojectinvoice create_project_invoice(project_uuid, create_project_invoice, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createprojectinvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**create_project_invoice** | [**CreateProjectInvoice**](CreateProjectInvoice.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseCreateprojectinvoice**](Response_Createprojectinvoice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_key

> crate::models::CreateKeyResponse create_project_key(project_uuid, security_key_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createprojectkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**security_key_create** | [**SecurityKeyCreate**](SecurityKeyCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::CreateKeyResponse**](CreateKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_usage_type

> crate::models::ProjectsUsageTypeDb create_usage_type(projects_usage_type_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createusagetype

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_usage_type_create** | [**ProjectsUsageTypeCreate**](ProjectsUsageTypeCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsUsageTypeDb**](ProjectsUsageTypeDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_credential

> crate::models::ResponseDeleteprojectcredential delete_project_credential(project_uuid, service_name, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteprojectcredential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**service_name** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseDeleteprojectcredential**](Response_Deleteprojectcredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_key

> String delete_project_key(project_uuid, access_token, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteprojectkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**access_token** | Option<**String**> |  |  |
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


## delete_usage_type

> crate::models::ResponseDeleteusagetype delete_usage_type(usage_type_key, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteusagetype

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usage_type_key** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseDeleteusagetype**](Response_Deleteusagetype.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_project_credentials

> Vec<crate::models::GetProjectCredential> get_all_project_credentials(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getallprojectcredentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::GetProjectCredential>**](GetProjectCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_project_credits

> crate::models::Page get_all_project_credits(project_uuid, fully_consumed, revoked, page, page_size, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getallprojectcredits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**fully_consumed** | Option<**bool**> |  |  |[default to false]
**revoked** | Option<**bool**> |  |  |[default to false]
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 25]
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


## get_all_project_usage

> Vec<crate::models::ProjectsProjectUsageDb> get_all_project_usage(project_uuid, year, month, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getallprojectusage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ProjectsProjectUsageDb>**](ProjectsProjectUsageDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_member_projects

> Vec<crate::models::ProjectsProjectGet> get_member_projects(entity_uuid, role, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getmemberprojects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_uuid** | **String** |  | [required] |
**role** | Option<**String**> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ProjectsProjectGet>**](ProjectsProjectGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> crate::models::ProjectDb get_project(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getproject

Get a Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectDb**](ProjectDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_credit_transactions

> crate::models::Page get_project_credit_transactions(project_uuid, credit_uuid, page, page_size, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getprojectcredittransactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**credit_uuid** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 25]
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


## get_project_invoice

> crate::models::GetProjectInvoiceResponse get_project_invoice(project_uuid, with_invoice, year, month, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getprojectinvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**with_invoice** | Option<**bool**> |  |  |[default to false]
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::GetProjectInvoiceResponse**](GetProjectInvoiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_invoice_history

> crate::models::GetProjectInvoiceHistory get_project_invoice_history(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getprojectinvoicehistory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::GetProjectInvoiceHistory**](GetProjectInvoiceHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_keys

> Vec<crate::models::ProjectsProjectMemberDb> get_project_keys(project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getprojectkeys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ProjectsProjectMemberDb>**](ProjectsProjectMemberDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_members

> Vec<crate::models::ProjectsProjectMemberDb> get_project_members(project_uuid, role, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getprojectmembers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**role** | Option<**String**> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ProjectsProjectMemberDb>**](ProjectsProjectMemberDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specific_project_credential

> crate::models::GetProjectCredential get_specific_project_credential(project_uuid, service_name, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getspecificprojectcredential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**service_name** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::GetProjectCredential**](GetProjectCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specific_project_usage

> crate::models::ProjectsProjectUsageDb get_specific_project_usage(usage_type_key, project_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getspecificprojectusage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usage_type_key** | **String** |  | [required] |
**project_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsProjectUsageDb**](ProjectsProjectUsageDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_type

> crate::models::ProjectsUsageTypeGet get_usage_type(usage_type_key)
Getusagetype

Get a UsageType  No auth because we may want to use this on pricing/docs pages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usage_type_key** | **String** |  | [required] |

### Return type

[**crate::models::ProjectsUsageTypeGet**](ProjectsUsageTypeGet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_member_from_project

> crate::models::ResponseRemovememberfromproject remove_member_from_project(project_uuid, entity_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Removememberfromproject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**entity_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseRemovememberfromproject**](Response_Removememberfromproject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_project_credit

> crate::models::ResponseRevokeprojectcredit revoke_project_credit(project_uuid, credit_uuid, revoked_reason, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Revokeprojectcredit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**credit_uuid** | **String** |  | [required] |
**revoked_reason** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseRevokeprojectcredit**](Response_Revokeprojectcredit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_projects

> crate::models::Page search_projects(is_active, page, page_size, search, search_on, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Searchprojects

Search projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_active** | Option<**bool**> |  |  |[default to false]
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 25]
**search** | Option<**String**> |  |  |
**search_on** | Option<**String**> |  |  |
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


## search_usage_type

> crate::models::Page search_usage_type(page, page_size, search, search_on, sort_on, sort_desc)
Searchusagetype

Get a UsageType  No auth because we may want to use this on pricing/docs pages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 25]
**search** | Option<**String**> |  |  |
**search_on** | Option<**String**> |  |  |
**sort_on** | Option<**String**> |  |  |
**sort_desc** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::Page**](Page.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> serde_json::Value update_project(project_uuid, projects_project_update, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updateproject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**projects_project_update** | [**ProjectsProjectUpdate**](ProjectsProjectUpdate.md) |  | [required] |
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


## update_project_credential

> crate::models::ResponseUpdateprojectcredential update_project_credential(project_uuid, service_name, update_project_credential_request, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updateprojectcredential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **String** |  | [required] |
**service_name** | **String** |  | [required] |
**update_project_credential_request** | [**UpdateProjectCredentialRequest**](UpdateProjectCredentialRequest.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseUpdateprojectcredential**](Response_Updateprojectcredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_usage_type

> crate::models::ProjectsUsageTypeDb update_usage_type(usage_type_key, projects_usage_type_update, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updateusagetype

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usage_type_key** | **String** |  | [required] |
**projects_usage_type_update** | [**ProjectsUsageTypeUpdate**](ProjectsUsageTypeUpdate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectsUsageTypeDb**](ProjectsUsageTypeDB.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


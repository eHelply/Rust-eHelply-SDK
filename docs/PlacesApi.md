# \PlacesApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**advanced_search_places**](PlacesApi.md#advanced_search_places) | **GET** /places/search/places/string | Advancedsearchplaces
[**create_place_places_places_post**](PlacesApi.md#create_place_places_places_post) | **POST** /places/places | Create Place
[**delete_place**](PlacesApi.md#delete_place) | **DELETE** /places/places/{place_uuid} | Deleteplace
[**forward_geocoding_places_geocoding_forward_get**](PlacesApi.md#forward_geocoding_places_geocoding_forward_get) | **GET** /places/geocoding/forward | Forward Geocoding
[**get_place**](PlacesApi.md#get_place) | **GET** /places/places/{place_uuid} | Getplace
[**reverse_geocoding_places_geocoding_reverse_get**](PlacesApi.md#reverse_geocoding_places_geocoding_reverse_get) | **GET** /places/geocoding/reverse | Reverse Geocoding
[**search_places**](PlacesApi.md#search_places) | **GET** /places/places | Searchplaces
[**update_place**](PlacesApi.md#update_place) | **PUT** /places/places/{place_uuid} | Updateplace



## advanced_search_places

> crate::models::Page advanced_search_places(search_string, page, page_size, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Advancedsearchplaces

Search places by a search string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_string** | Option<**String**> |  |  |[default to ]
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


## create_place_places_places_post

> crate::models::PlaceResponse create_place_places_places_post(place_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Create Place

Creates a Place.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place_base** | [**PlaceBase**](PlaceBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::PlaceResponse**](PlaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_place

> serde_json::Value delete_place(place_uuid, soft_delete, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteplace

Deletes the place with the given ID and returns True if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place_uuid** | **String** |  | [required] |
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


## forward_geocoding_places_geocoding_forward_get

> serde_json::Value forward_geocoding_places_geocoding_forward_get(searching_place, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Forward Geocoding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**searching_place** | **String** |  | [required] |
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


## get_place

> crate::models::PlaceResponse get_place(place_uuid, with_meta, with_catalog, with_reviews, with_schedule, with_collection, with_blog, with_tags, with_categories, with_company, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getplace

Gets the place information given the Place ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place_uuid** | **String** |  | [required] |
**with_meta** | Option<**bool**> |  |  |[default to false]
**with_catalog** | Option<**bool**> |  |  |[default to false]
**with_reviews** | Option<**bool**> |  |  |[default to false]
**with_schedule** | Option<**bool**> |  |  |[default to false]
**with_collection** | Option<**bool**> |  |  |[default to false]
**with_blog** | Option<**bool**> |  |  |[default to false]
**with_tags** | Option<**bool**> |  |  |[default to false]
**with_categories** | Option<**bool**> |  |  |[default to false]
**with_company** | Option<**bool**> |  |  |[default to false]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::PlaceResponse**](PlaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reverse_geocoding_places_geocoding_reverse_get

> serde_json::Value reverse_geocoding_places_geocoding_reverse_get(long, lat, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Reverse Geocoding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**long** | **f32** |  | [required] |
**lat** | **f32** |  | [required] |
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


## search_places

> crate::models::Page search_places(name, address_line_1, address_line_2, city, province_state, country, postal_zip_code, lat, lng, email, is_public, is_deleted, with_company, with_meta, with_catalog, with_reviews, with_schedule, with_collection, with_blog, with_tags, with_categories, page, page_size, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Searchplaces

Search all places and returns paginated results with Places being stored in items field. Can search by `project_uuid, name, address, address_line_2, city, province_state, country, postal_zip_code, lat, lng email` string fields or the `is_public and is_deleted` boolean fields. To search with these fields use query params with string values. For sorting fill out \"sort_desc\" field with either true/false and the \"sort_on\" query parameter with column you want to sort on (ex: name). Max pagination items per page is 50. Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      meta_uuid                           **type:** string or None      catalog_data                        **type:** dict or None      review_group_data                   **type:** dict or None      schedule_data                       **type:** dict or None      collection_data                     **type:** dict or None      blog_data                           **type:** dict or None      tags                                **type:** [TagBase] or None      categories                          **type:** [CategoryBase] or None      company                             **type:** CompanyBase or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**address_line_1** | Option<**String**> |  |  |
**address_line_2** | Option<**String**> |  |  |
**city** | Option<**String**> |  |  |
**province_state** | Option<**String**> |  |  |
**country** | Option<**String**> |  |  |
**postal_zip_code** | Option<**String**> |  |  |
**lat** | Option<**String**> |  |  |
**lng** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |
**is_public** | Option<**bool**> |  |  |[default to true]
**is_deleted** | Option<**bool**> |  |  |[default to false]
**with_company** | Option<**bool**> |  |  |[default to false]
**with_meta** | Option<**bool**> |  |  |[default to false]
**with_catalog** | Option<**bool**> |  |  |[default to false]
**with_reviews** | Option<**bool**> |  |  |[default to false]
**with_schedule** | Option<**bool**> |  |  |[default to false]
**with_collection** | Option<**bool**> |  |  |[default to false]
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


## update_place

> crate::models::PlaceResponse update_place(place_uuid, place_base, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updateplace

Update Place with given info, only updating the fields supplied. Place Uuid must be sent however.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place_uuid** | **String** |  | [required] |
**place_base** | [**PlaceBase**](PlaceBase.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::PlaceResponse**](PlaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


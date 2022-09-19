# \MonitorApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_alarm**](MonitorApi.md#acknowledge_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/acknowledge | Acknowledgealarm
[**assign_alarm**](MonitorApi.md#assign_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/assign | Assignalarm
[**attach_alarm_note**](MonitorApi.md#attach_alarm_note) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/note | Attachalarmnote
[**attach_alarm_ticket**](MonitorApi.md#attach_alarm_ticket) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/ticket | Attachalarmticket
[**clear_alarm**](MonitorApi.md#clear_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/clear | Clearalarm
[**get_service**](MonitorApi.md#get_service) | **GET** /sam/monitor/services/{service} | Getservice
[**get_service_alarm**](MonitorApi.md#get_service_alarm) | **GET** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid} | Getservicealarm
[**get_service_alarms**](MonitorApi.md#get_service_alarms) | **GET** /sam/monitor/services/{service}/stages/{stage}/alarms | Getservicealarms
[**get_service_heartbeat**](MonitorApi.md#get_service_heartbeat) | **GET** /sam/monitor/services/{service}/stages/{stage}/heartbeats | Getserviceheartbeat
[**get_service_kpis**](MonitorApi.md#get_service_kpis) | **GET** /sam/monitor/services/{service}/kpis | Getservicekpis
[**get_service_spec**](MonitorApi.md#get_service_spec) | **GET** /sam/monitor/services/{service}/specs/{spec} | Getservicespec
[**get_service_specs**](MonitorApi.md#get_service_specs) | **GET** /sam/monitor/services/{service}/specs | Getservicespecs
[**get_service_vitals**](MonitorApi.md#get_service_vitals) | **GET** /sam/monitor/services/{service}/stages/{stage}/vitals | Getservicevitals
[**get_services**](MonitorApi.md#get_services) | **GET** /sam/monitor/services | Getservices
[**get_services_with_specs**](MonitorApi.md#get_services_with_specs) | **GET** /sam/monitor/specs/services | Getserviceswithspecs
[**hide_service**](MonitorApi.md#hide_service) | **POST** /sam/monitor/services/{service}/stages/{stage}/hide | Hideservice
[**ignore_alarm**](MonitorApi.md#ignore_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/ignore | Ignorealarm
[**register_service**](MonitorApi.md#register_service) | **POST** /sam/monitor/services | Registerservice
[**search_alarms**](MonitorApi.md#search_alarms) | **GET** /sam/monitor/services/{service}/alarms | Searchalarms
[**show_service**](MonitorApi.md#show_service) | **POST** /sam/monitor/services/{service}/stages/{stage}/show | Showservice
[**terminate_alarm**](MonitorApi.md#terminate_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/terminate | Terminatealarm
[**trigger_alarm**](MonitorApi.md#trigger_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms | Triggeralarm



## acknowledge_alarm

> crate::models::AlarmResponse acknowledge_alarm(service, stage, alarm_uuid, alarm_acknowledge)
Acknowledgealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_acknowledge** | [**AlarmAcknowledge**](AlarmAcknowledge.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_alarm

> crate::models::AlarmResponse assign_alarm(service, stage, alarm_uuid, alarm_assign)
Assignalarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_assign** | [**AlarmAssign**](AlarmAssign.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_alarm_note

> crate::models::AlarmResponse attach_alarm_note(service, stage, alarm_uuid, alarm_note)
Attachalarmnote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_note** | [**AlarmNote**](AlarmNote.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_alarm_ticket

> crate::models::AlarmResponse attach_alarm_ticket(service, stage, alarm_uuid, alarm_ticket)
Attachalarmticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_ticket** | [**AlarmTicket**](AlarmTicket.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_alarm

> crate::models::AlarmResponse clear_alarm(service, stage, alarm_uuid)
Clearalarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service

> crate::models::ServiceResponse get_service(service, heartbeats, heartbeat_limit, alarms, alarm_limit, stage)
Getservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**heartbeats** | Option<**bool**> |  |  |[default to false]
**heartbeat_limit** | Option<**i32**> |  |  |[default to 5]
**alarms** | Option<**bool**> |  |  |[default to false]
**alarm_limit** | Option<**i32**> |  |  |[default to 5]
**stage** | Option<**String**> |  |  |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_alarm

> crate::models::AlarmResponse get_service_alarm(service, stage, alarm_uuid)
Getservicealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_alarms

> Vec<crate::models::AlarmResponse> get_service_alarms(service, stage, history, include_terminated, include_cleared)
Getservicealarms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]
**include_terminated** | Option<**bool**> |  |  |[default to false]
**include_cleared** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<crate::models::AlarmResponse>**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_heartbeat

> Vec<crate::models::HeartbeatResponse> get_service_heartbeat(service, stage, history)
Getserviceheartbeat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]

### Return type

[**Vec<crate::models::HeartbeatResponse>**](HeartbeatResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_kpis

> Vec<crate::models::KpiResponse> get_service_kpis(service, history)
Getservicekpis

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]

### Return type

[**Vec<crate::models::KpiResponse>**](KpiResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_spec

> crate::models::GetServiceSpecResponse get_service_spec(service, spec)
Getservicespec

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**spec** | **String** |  | [required] |

### Return type

[**crate::models::GetServiceSpecResponse**](GetServiceSpecResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_specs

> crate::models::GetServiceSpecsResponse get_service_specs(service)
Getservicespecs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |

### Return type

[**crate::models::GetServiceSpecsResponse**](GetServiceSpecsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_vitals

> Vec<crate::models::StatsVitalsResponse> get_service_vitals(service, stage, history)
Getservicevitals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]

### Return type

[**Vec<crate::models::StatsVitalsResponse>**](StatsVitalsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> Vec<crate::models::ServiceResponse> get_services(heartbeats, heartbeat_limit, alarms, alarm_limit, include_hidden, stage, key)
Getservices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**heartbeats** | Option<**bool**> |  |  |[default to false]
**heartbeat_limit** | Option<**i32**> |  |  |[default to 5]
**alarms** | Option<**bool**> |  |  |[default to false]
**alarm_limit** | Option<**i32**> |  |  |[default to 5]
**include_hidden** | Option<**bool**> |  |  |[default to false]
**stage** | Option<**String**> |  |  |
**key** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ServiceResponse>**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services_with_specs

> crate::models::GetServiceServiceWithSpecsResponse get_services_with_specs()
Getserviceswithspecs

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetServiceServiceWithSpecsResponse**](GetServiceServiceWithSpecsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_service

> crate::models::ServiceMessageResponse hide_service(service, stage)
Hideservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |

### Return type

[**crate::models::ServiceMessageResponse**](ServiceMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ignore_alarm

> crate::models::AlarmResponse ignore_alarm(service, stage, alarm_uuid, alarm_ignore)
Ignorealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_ignore** | [**AlarmIgnore**](AlarmIgnore.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_service

> crate::models::ServiceResponse register_service(service_create)
Registerservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_create** | [**ServiceCreate**](ServiceCreate.md) |  | [required] |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_alarms

> crate::models::Page search_alarms(service, page, page_size, search, search_on, sort_on, sort_desc)
Searchalarms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
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


## show_service

> crate::models::ServiceMessageResponse show_service(service, stage)
Showservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |

### Return type

[**crate::models::ServiceMessageResponse**](ServiceMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_alarm

> crate::models::AlarmResponse terminate_alarm(service, stage, alarm_uuid, alarm_terminate)
Terminatealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_terminate** | [**AlarmTerminate**](AlarmTerminate.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_alarm

> crate::models::AlarmResponse trigger_alarm(service, stage, alarm_create)
Triggeralarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_create** | [**AlarmCreate**](AlarmCreate.md) |  | [required] |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


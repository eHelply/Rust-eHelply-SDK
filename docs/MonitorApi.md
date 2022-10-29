# \MonitorApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_alarm**](MonitorApi.md#acknowledge_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/acknowledge | Acknowledgealarm
[**assign_alarm**](MonitorApi.md#assign_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/assign | Assignalarm
[**attach_alarm_note**](MonitorApi.md#attach_alarm_note) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/note | Attachalarmnote
[**attach_alarm_ticket**](MonitorApi.md#attach_alarm_ticket) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/ticket | Attachalarmticket
[**clear_alarm**](MonitorApi.md#clear_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/clear | Clearalarm
[**delete_service_super_stack_meta**](MonitorApi.md#delete_service_super_stack_meta) | **DELETE** /sam/monitor/services/{service}/superstack | Deleteservicesuperstackmeta
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
[**get_supertack_services**](MonitorApi.md#get_supertack_services) | **GET** /sam/monitor/superstack-services | Getsupertackservices
[**hide_service**](MonitorApi.md#hide_service) | **POST** /sam/monitor/services/{service}/stages/{stage}/hide | Hideservice
[**ignore_alarm**](MonitorApi.md#ignore_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/ignore | Ignorealarm
[**register_service**](MonitorApi.md#register_service) | **POST** /sam/monitor/services | Registerservice
[**save_service_super_stack_meta**](MonitorApi.md#save_service_super_stack_meta) | **POST** /sam/monitor/services/{service}/superstack | Saveservicesuperstackmeta
[**search_alarms**](MonitorApi.md#search_alarms) | **GET** /sam/monitor/services/{service}/alarms | Searchalarms
[**show_service**](MonitorApi.md#show_service) | **POST** /sam/monitor/services/{service}/stages/{stage}/show | Showservice
[**terminate_alarm**](MonitorApi.md#terminate_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/terminate | Terminatealarm
[**trigger_alarm**](MonitorApi.md#trigger_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms | Triggeralarm



## acknowledge_alarm

> crate::models::AlarmResponse acknowledge_alarm(service, stage, alarm_uuid, alarm_acknowledge, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Acknowledgealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_acknowledge** | [**AlarmAcknowledge**](AlarmAcknowledge.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_alarm

> crate::models::AlarmResponse assign_alarm(service, stage, alarm_uuid, alarm_assign, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Assignalarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_assign** | [**AlarmAssign**](AlarmAssign.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_alarm_note

> crate::models::AlarmResponse attach_alarm_note(service, stage, alarm_uuid, alarm_note, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Attachalarmnote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_note** | [**AlarmNote**](AlarmNote.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_alarm_ticket

> crate::models::AlarmResponse attach_alarm_ticket(service, stage, alarm_uuid, alarm_ticket, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Attachalarmticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_ticket** | [**AlarmTicket**](AlarmTicket.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_alarm

> crate::models::AlarmResponse clear_alarm(service, stage, alarm_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Clearalarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_super_stack_meta

> serde_json::Value delete_service_super_stack_meta(service, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteservicesuperstackmeta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
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


## get_service

> crate::models::ServiceResponse get_service(service, heartbeats, heartbeat_limit, alarms, alarm_limit, stage, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
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
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_alarm

> crate::models::AlarmResponse get_service_alarm(service, stage, alarm_uuid, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getservicealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_alarms

> Vec<crate::models::AlarmResponse> get_service_alarms(service, stage, history, include_terminated, include_cleared, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getservicealarms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]
**include_terminated** | Option<**bool**> |  |  |[default to false]
**include_cleared** | Option<**bool**> |  |  |[default to false]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AlarmResponse>**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_heartbeat

> Vec<crate::models::HeartbeatResponse> get_service_heartbeat(service, stage, history, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getserviceheartbeat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::HeartbeatResponse>**](HeartbeatResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_kpis

> Vec<crate::models::KpiResponse> get_service_kpis(service, history, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getservicekpis

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

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

> Vec<crate::models::StatsVitalsResponse> get_service_vitals(service, stage, history, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getservicevitals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**history** | Option<**i32**> |  |  |[default to 5]
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::StatsVitalsResponse>**](StatsVitalsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> Vec<crate::models::ServiceResponse> get_services(heartbeats, heartbeat_limit, alarms, alarm_limit, include_hidden, stage, key, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
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
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

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


## get_supertack_services

> Vec<crate::models::ServiceSuperStackMeta> get_supertack_services()
Getsupertackservices

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ServiceSuperStackMeta>**](ServiceSuperStackMeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_service

> crate::models::ServiceMessageResponse hide_service(service, stage, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Hideservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ServiceMessageResponse**](ServiceMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ignore_alarm

> crate::models::AlarmResponse ignore_alarm(service, stage, alarm_uuid, alarm_ignore, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Ignorealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_ignore** | [**AlarmIgnore**](AlarmIgnore.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_service

> crate::models::ServiceResponse register_service(service_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Registerservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_create** | [**ServiceCreate**](ServiceCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_service_super_stack_meta

> serde_json::Value save_service_super_stack_meta(service, service_super_stack_meta, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Saveservicesuperstackmeta

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**service_super_stack_meta** | [**ServiceSuperStackMeta**](ServiceSuperStackMeta.md) |  | [required] |
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


## search_alarms

> crate::models::Page search_alarms(service, page, page_size, search, search_on, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
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


## show_service

> crate::models::ServiceMessageResponse show_service(service, stage, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Showservice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ServiceMessageResponse**](ServiceMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_alarm

> crate::models::AlarmResponse terminate_alarm(service, stage, alarm_uuid, alarm_terminate, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Terminatealarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_uuid** | **String** |  | [required] |
**alarm_terminate** | [**AlarmTerminate**](AlarmTerminate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_alarm

> crate::models::AlarmResponse trigger_alarm(service, stage, alarm_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Triggeralarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**stage** | **String** |  | [required] |
**alarm_create** | [**AlarmCreate**](AlarmCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::AlarmResponse**](AlarmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \UsersApi

All URIs are relative to *https://api.prod.ehelply.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_signup**](UsersApi.md#confirm_signup) | **POST** /sam/users/auth/signup/confirm | Confirmsignup
[**create_participant**](UsersApi.md#create_participant) | **POST** /sam/users/participants | Createparticipant
[**create_user**](UsersApi.md#create_user) | **POST** /sam/users | Createuser
[**delete_participant**](UsersApi.md#delete_participant) | **DELETE** /sam/users/participants/{participant_id} | Deleteparticipant
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /sam/users/{user_id} | Deleteuser
[**get_participant**](UsersApi.md#get_participant) | **GET** /sam/users/participants/{participant_id} | Getparticipant
[**get_user**](UsersApi.md#get_user) | **GET** /sam/users/{user_id} | Getuser
[**login**](UsersApi.md#login) | **POST** /sam/users/auth/login | Login
[**refresh_token**](UsersApi.md#refresh_token) | **POST** /sam/users/auth/{app_client}/refresh-token | Refreshtoken
[**reset_password**](UsersApi.md#reset_password) | **POST** /sam/users/auth/password/reset | Resetpassword
[**reset_password_confirmation**](UsersApi.md#reset_password_confirmation) | **POST** /sam/users/auth/password/reset/confirm | Resetpasswordconfirmation
[**search_participants**](UsersApi.md#search_participants) | **GET** /sam/users/participants | Searchparticipants
[**signup**](UsersApi.md#signup) | **POST** /sam/users/auth/signup | Signup
[**update_participant**](UsersApi.md#update_participant) | **PUT** /sam/users/participants/{participant_id} | Updateparticipant
[**update_user**](UsersApi.md#update_user) | **PUT** /sam/users/{user_id} | Updateuser
[**user_validations**](UsersApi.md#user_validations) | **POST** /sam/users/validations/{field} | Uservalidations



## confirm_signup

> serde_json::Value confirm_signup(user_confirmation)
Confirmsignup

Validates a user signup with a given confirmation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_confirmation** | [**UserConfirmation**](UserConfirmation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_participant

> crate::models::ParticipantUserReturn create_participant(participant_create, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Createparticipant

Creates a participant given the participant info (meta and user_id)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_create** | [**ParticipantCreate**](ParticipantCreate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ParticipantUserReturn**](ParticipantUserReturn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::UserResponse create_user(authorization)
Createuser

Usually ran after login and will do the following: - If no user exists (AKA signed in with social media) it will create a new user and default participant - If a user exists, sync Cognito data from Cognito to the user - Determine missing fields that SHOULD be filled

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<**String**> |  |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_participant

> bool delete_participant(participant_id, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteparticipant

Delete participants related to the given participant_id, returns True if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> bool delete_user(user_id, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Deleteuser

Soft deletes the user with the provided user id, granted the deleter is the same person or an admin. Returns True if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_participant

> crate::models::ParticipantUserReturn get_participant(participant_id, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getparticipant

Gets a participant given their participant ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **String** |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ParticipantUserReturn**](ParticipantUserReturn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::UserResponse get_user(user_id, id_type, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Getuser

Gets the user object given user id (uuid) or cognito id (cognito)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**id_type** | Option<**String**> |  |  |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> crate::models::UserLoginReturn login(user_login)
Login

Login endpoint, returns tokens. EMAIL NEEDS TO BE VERIFIED (can be done through the email the user received).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_login** | [**UserLogin**](UserLogin.md) |  | [required] |

### Return type

[**crate::models::UserLoginReturn**](UserLoginReturn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_token

> crate::models::UserTokenReturn refresh_token(app_client, body)
Refreshtoken

Refreshes tokens given a refresh token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_client** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

[**crate::models::UserTokenReturn**](UserTokenReturn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> serde_json::Value reset_password(user_password_reset)
Resetpassword

Sends the user an email with a confirmation code so they can reset their password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_password_reset** | [**UserPasswordReset**](UserPasswordReset.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password_confirmation

> serde_json::Value reset_password_confirmation(user_password_reset_confirmation)
Resetpasswordconfirmation

Resets the given user's password to the given password when the proper code is provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_password_reset_confirmation** | [**UserPasswordResetConfirmation**](UserPasswordResetConfirmation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_participants

> crate::models::Page search_participants(page, page_size, search, search_on, sort_on, sort_desc, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Searchparticipants

Search participants using a user uuid, returns pagination information and list of `items` (ParticipantUserReturn from GET Participant). Can search on \"user_uuid\", and sort on any field. To search enter search value into \"search\" query param and the field into \"search on\" (currently only \"user\"uuid\"). For sorting fill out \"sort_desc\" field with either true/false and the \"sort_on\" query parameter with column you want to sort on (ex: date_created). Max pagination items per page is 50.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## signup

> crate::models::UserSignupReturn signup(user_signup)
Signup

Signup to eHelply, creates a user and default participant behind the scenes. Does not verify email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_signup** | [**UserSignup**](UserSignup.md) |  | [required] |

### Return type

[**crate::models::UserSignupReturn**](UserSignupReturn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_participant

> crate::models::ParticipantUserReturn update_participant(participant_id, participant_update, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updateparticipant

Update participant data given

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **String** |  | [required] |
**participant_update** | [**ParticipantUpdate**](ParticipantUpdate.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::ParticipantUserReturn**](ParticipantUserReturn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserResponse update_user(user_id, user, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Updateuser

Update the given user and sync the cognito data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**user** | [**User**](User.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_validations

> bool user_validations(field, user_validations, x_access_token, x_secret_token, authorization, ehelply_active_participant, ehelply_project, ehelply_data)
Uservalidations

Validates a certain field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** |  | [required] |
**user_validations** | [**UserValidations**](UserValidations.md) |  | [required] |
**x_access_token** | Option<**String**> |  |  |
**x_secret_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> |  |  |
**ehelply_active_participant** | Option<**String**> |  |  |
**ehelply_project** | Option<**String**> |  |  |
**ehelply_data** | Option<**String**> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


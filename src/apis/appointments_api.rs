/*
 * eHelply SDK - 1.1.120
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.120
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`add_entity_to_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddEntityToAppointmentError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAppointmentError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAppointmentError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`detach_entity_from_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachEntityFromAppointmentError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppointmentError {
    Status403(crate::models::GetAppointment403Response),
    Status404(crate::models::GetAppointment403Response),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchAppointmentError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_appointment_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchAppointmentEntitiesError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_entity_appointments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchEntityAppointmentsError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_appointment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppointmentError {
    Status403(crate::models::GetAppointment403Response),
    Status404(crate::models::GetAppointment403Response),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


pub async fn add_entity_to_appointment(configuration: &configuration::Configuration, appointment_uuid: &str, entity_uuid: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<bool, Error<AddEntityToAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/{appointment_uuid}/entities/{entity_uuid}", local_var_configuration.base_path, appointment_uuid=crate::apis::urlencode(appointment_uuid), entity_uuid=crate::apis::urlencode(entity_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddEntityToAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_appointment(configuration: &configuration::Configuration, appointment_base: crate::models::AppointmentBase, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::AppointmentResponse, Error<CreateAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&appointment_base);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_appointment(configuration: &configuration::Configuration, appointment_uuid: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<bool, Error<DeleteAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/{appointment_uuid}", local_var_configuration.base_path, appointment_uuid=crate::apis::urlencode(appointment_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn detach_entity_from_appointment(configuration: &configuration::Configuration, appointment_uuid: &str, entity_uuid: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<bool, Error<DetachEntityFromAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/{appointment_uuid}/entities/{entity_uuid}", local_var_configuration.base_path, appointment_uuid=crate::apis::urlencode(appointment_uuid), entity_uuid=crate::apis::urlencode(entity_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DetachEntityFromAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_appointment(configuration: &configuration::Configuration, appointment_uuid: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::AppointmentResponse, Error<GetAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/{appointment_uuid}", local_var_configuration.base_path, appointment_uuid=crate::apis::urlencode(appointment_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn search_appointment(configuration: &configuration::Configuration, place_uuid: Option<&str>, exclude_cancelled: Option<bool>, is_deleted: Option<bool>, start_range: Option<&str>, end_range: Option<&str>, page: Option<i32>, page_size: Option<i32>, sort_on: Option<&str>, sort_desc: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<SearchAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = place_uuid {
        local_var_req_builder = local_var_req_builder.query(&[("place_uuid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_cancelled {
        local_var_req_builder = local_var_req_builder.query(&[("exclude_cancelled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_deleted {
        local_var_req_builder = local_var_req_builder.query(&[("is_deleted", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_range {
        local_var_req_builder = local_var_req_builder.query(&[("start_range", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_range {
        local_var_req_builder = local_var_req_builder.query(&[("end_range", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_on {
        local_var_req_builder = local_var_req_builder.query(&[("sort_on", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_desc {
        local_var_req_builder = local_var_req_builder.query(&[("sort_desc", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn search_appointment_entities(configuration: &configuration::Configuration, appointment_uuid: &str, page: Option<i32>, page_size: Option<i32>, sort_on: Option<&str>, sort_desc: Option<bool>, search: Option<&str>, search_on: Option<&str>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<SearchAppointmentEntitiesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/{appointment_uuid}/entities", local_var_configuration.base_path, appointment_uuid=crate::apis::urlencode(appointment_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_on {
        local_var_req_builder = local_var_req_builder.query(&[("sort_on", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_desc {
        local_var_req_builder = local_var_req_builder.query(&[("sort_desc", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search_on {
        local_var_req_builder = local_var_req_builder.query(&[("search_on", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchAppointmentEntitiesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn search_entity_appointments(configuration: &configuration::Configuration, entity_uuid: &str, start_date: Option<&str>, end_date: Option<&str>, exclude_cancelled: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<SearchEntityAppointmentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/entities/{entity_uuid}/appointments", local_var_configuration.base_path, entity_uuid=crate::apis::urlencode(entity_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start_date {
        local_var_req_builder = local_var_req_builder.query(&[("start_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date {
        local_var_req_builder = local_var_req_builder.query(&[("end_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_cancelled {
        local_var_req_builder = local_var_req_builder.query(&[("exclude_cancelled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchEntityAppointmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_appointment(configuration: &configuration::Configuration, appointment_uuid: &str, appointment_base: crate::models::AppointmentBase, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::AppointmentResponse, Error<UpdateAppointmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appointments/appointments/{appointment_uuid}", local_var_configuration.base_path, appointment_uuid=crate::apis::urlencode(appointment_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_access_token {
        local_var_req_builder = local_var_req_builder.header("x-access-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_secret_token {
        local_var_req_builder = local_var_req_builder.header("x-secret-token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_active_participant {
        local_var_req_builder = local_var_req_builder.header("ehelply-active-participant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_project {
        local_var_req_builder = local_var_req_builder.header("ehelply-project", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ehelply_data {
        local_var_req_builder = local_var_req_builder.header("ehelply-data", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&appointment_base);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateAppointmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


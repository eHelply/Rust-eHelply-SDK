/*
 * eHelply SDK - 1.1.116
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.116
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContactError {
    Status403(crate::models::GetAppointment403Response),
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_ticket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTicketError {
    Status403(crate::models::GetAppointment403Response),
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tickets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTicketsError {
    Status403(crate::models::GetAppointment403Response),
    Status404(crate::models::GetAppointment403Response),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_ticket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTicketError {
    Status403(crate::models::GetAppointment403Response),
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`view_ticket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ViewTicketError {
    Status403(crate::models::GetAppointment403Response),
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


pub async fn create_contact(configuration: &configuration::Configuration, contact: crate::models::Contact, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::ContactResponse, Error<CreateContactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sam/support/contact", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&contact);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateContactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_ticket(configuration: &configuration::Configuration, project_uuid: &str, member_uuid: &str, create_ticket: crate::models::CreateTicket, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::TicketResponse, Error<CreateTicketError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sam/support/projects/{project_uuid}/members/{member_uuid}/tickets", local_var_configuration.base_path, project_uuid=crate::apis::urlencode(project_uuid), member_uuid=crate::apis::urlencode(member_uuid));
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
    local_var_req_builder = local_var_req_builder.json(&create_ticket);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTicketError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_tickets(configuration: &configuration::Configuration, project_uuid: &str, member_uuid: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<Vec<crate::models::TicketsResponse>, Error<ListTicketsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sam/support/projects/{project_uuid}/members/{member_uuid}/tickets", local_var_configuration.base_path, project_uuid=crate::apis::urlencode(project_uuid), member_uuid=crate::apis::urlencode(member_uuid));
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
        let local_var_entity: Option<ListTicketsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_ticket(configuration: &configuration::Configuration, project_uuid: &str, member_uuid: &str, ticket_id: &str, create_ticket: crate::models::CreateTicket, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::TicketResponse, Error<UpdateTicketError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sam/support/projects/{project_uuid}/members/{member_uuid}/tickets/{ticket_id}", local_var_configuration.base_path, project_uuid=crate::apis::urlencode(project_uuid), member_uuid=crate::apis::urlencode(member_uuid), ticket_id=crate::apis::urlencode(ticket_id));
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
    local_var_req_builder = local_var_req_builder.json(&create_ticket);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateTicketError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn view_ticket(configuration: &configuration::Configuration, project_uuid: &str, member_uuid: &str, ticket_id: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::TicketResponse, Error<ViewTicketError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sam/support/projects/{project_uuid}/members/{member_uuid}/tickets/{ticket_id}", local_var_configuration.base_path, project_uuid=crate::apis::urlencode(project_uuid), member_uuid=crate::apis::urlencode(member_uuid), ticket_id=crate::apis::urlencode(ticket_id));
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
        let local_var_entity: Option<ViewTicketError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


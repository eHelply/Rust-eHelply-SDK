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


/// struct for typed errors of method [`delete_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Deletes the tag member with the given ID and returns True if successful
pub async fn delete_tag(configuration: &configuration::Configuration, tag_uuid: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<DeleteTagError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/tags/{tag_uuid}", local_var_configuration.base_path, tag_uuid=crate::apis::urlencode(tag_uuid));
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
        let local_var_entity: Option<DeleteTagError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


/*
 * eHelply SDK - 1.1.118
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.118
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_company_places_companies_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompanyPlacesCompaniesPostError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_place_places_companies_company_uuid_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePlacePlacesCompaniesCompanyUuidDeleteError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_company_places_companies_company_uuid_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompanyPlacesCompaniesCompanyUuidGetError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_companies_places_companies_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCompaniesPlacesCompaniesGetError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_company_places_companies_company_uuid_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCompanyPlacesCompaniesCompanyUuidPutError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Creates a company
pub async fn create_company_places_companies_post(configuration: &configuration::Configuration, company_base: crate::models::CompanyBase, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::CompanyResponse, Error<CreateCompanyPlacesCompaniesPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/companies", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&company_base);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCompanyPlacesCompaniesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the company with the given ID and returns True if successful
pub async fn delete_place_places_companies_company_uuid_delete(configuration: &configuration::Configuration, company_uuid: &str, soft_delete: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<DeletePlacePlacesCompaniesCompanyUuidDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/companies/{company_uuid}", local_var_configuration.base_path, company_uuid=crate::apis::urlencode(company_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = soft_delete {
        local_var_req_builder = local_var_req_builder.query(&[("soft_delete", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DeletePlacePlacesCompaniesCompanyUuidDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets the company information given the Place ID
pub async fn get_company_places_companies_company_uuid_get(configuration: &configuration::Configuration, company_uuid: &str, with_meta: Option<bool>, with_catalog: Option<bool>, with_reviews: Option<bool>, with_schedule: Option<bool>, with_blog: Option<bool>, with_tags: Option<bool>, with_categories: Option<bool>, with_places: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::CompanyResponse, Error<GetCompanyPlacesCompaniesCompanyUuidGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/companies/{company_uuid}", local_var_configuration.base_path, company_uuid=crate::apis::urlencode(company_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = with_meta {
        local_var_req_builder = local_var_req_builder.query(&[("with_meta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_catalog {
        local_var_req_builder = local_var_req_builder.query(&[("with_catalog", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_reviews {
        local_var_req_builder = local_var_req_builder.query(&[("with_reviews", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_schedule {
        local_var_req_builder = local_var_req_builder.query(&[("with_schedule", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_blog {
        local_var_req_builder = local_var_req_builder.query(&[("with_blog", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_tags {
        local_var_req_builder = local_var_req_builder.query(&[("with_tags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_categories {
        local_var_req_builder = local_var_req_builder.query(&[("with_categories", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_places {
        local_var_req_builder = local_var_req_builder.query(&[("with_places", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetCompanyPlacesCompaniesCompanyUuidGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search all companies and returns paginated results with Companies being stored in items field. Can search by `project_uuid, name, email` string fields or the `is_public and is_deleted` boolean fields. To search with these fields use query params with string values. For sorting fill out \"sort_desc\" field with either true/false and the \"sort_on\" query parameter with column you want to sort on (ex: name). Max pagination items per page is 50. Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      meta_uuid                           **type:** string or None      catalog_data                        **type:** dict or None      review_group_data                   **type:** dict or None      schedule_data                       **type:** dict or None      blog_data                           **type:** dict or None      tags                                **type:** [TagBase] or None      categories                          **type:** [CategoryBase] or None      places                              **type:** PlaceBase or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```
pub async fn search_companies_places_companies_get(configuration: &configuration::Configuration, project_uuid: Option<&str>, name: Option<&str>, email: Option<&str>, is_public: Option<bool>, is_deleted: Option<bool>, with_places: Option<bool>, with_meta: Option<bool>, with_catalog: Option<bool>, with_reviews: Option<bool>, with_schedule: Option<bool>, with_blog: Option<bool>, with_tags: Option<bool>, with_categories: Option<bool>, page: Option<i32>, page_size: Option<i32>, sort_on: Option<&str>, sort_desc: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::Page, Error<SearchCompaniesPlacesCompaniesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/companies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = project_uuid {
        local_var_req_builder = local_var_req_builder.query(&[("project_uuid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = email {
        local_var_req_builder = local_var_req_builder.query(&[("email", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_public {
        local_var_req_builder = local_var_req_builder.query(&[("is_public", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_deleted {
        local_var_req_builder = local_var_req_builder.query(&[("is_deleted", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_places {
        local_var_req_builder = local_var_req_builder.query(&[("with_places", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_meta {
        local_var_req_builder = local_var_req_builder.query(&[("with_meta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_catalog {
        local_var_req_builder = local_var_req_builder.query(&[("with_catalog", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_reviews {
        local_var_req_builder = local_var_req_builder.query(&[("with_reviews", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_schedule {
        local_var_req_builder = local_var_req_builder.query(&[("with_schedule", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_blog {
        local_var_req_builder = local_var_req_builder.query(&[("with_blog", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_tags {
        local_var_req_builder = local_var_req_builder.query(&[("with_tags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_categories {
        local_var_req_builder = local_var_req_builder.query(&[("with_categories", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SearchCompaniesPlacesCompaniesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update company with given info, only updating the fields supplied. Company Uuid must be sent however.
pub async fn update_company_places_companies_company_uuid_put(configuration: &configuration::Configuration, company_uuid: &str, company_base: crate::models::CompanyBase, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::CompanyResponse, Error<UpdateCompanyPlacesCompaniesCompanyUuidPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/companies/{company_uuid}", local_var_configuration.base_path, company_uuid=crate::apis::urlencode(company_uuid));
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
    local_var_req_builder = local_var_req_builder.json(&company_base);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateCompanyPlacesCompaniesCompanyUuidPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


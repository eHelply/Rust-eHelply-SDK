/*
 * eHelply SDK - 1.1.119
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.119
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`advanced_search_places`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdvancedSearchPlacesError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_place_places_places_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePlacePlacesPlacesPostError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_place`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePlaceError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forward_geocoding_places_geocoding_forward_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardGeocodingPlacesGeocodingForwardGetError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_place`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPlaceError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reverse_geocoding_places_geocoding_reverse_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReverseGeocodingPlacesGeocodingReverseGetError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_places`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchPlacesError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_place`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePlaceError {
    Status404(),
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Search places by a search string
pub async fn advanced_search_places(configuration: &configuration::Configuration, search_string: Option<&str>, page: Option<i32>, page_size: Option<i32>, sort_on: Option<&str>, sort_desc: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::Page, Error<AdvancedSearchPlacesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/search/places/string", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = search_string {
        local_var_req_builder = local_var_req_builder.query(&[("search_string", &local_var_str.to_string())]);
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
        let local_var_entity: Option<AdvancedSearchPlacesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a Place.
pub async fn create_place_places_places_post(configuration: &configuration::Configuration, place_base: crate::models::PlaceBase, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::PlaceResponse, Error<CreatePlacePlacesPlacesPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/places", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&place_base);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreatePlacePlacesPlacesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the place with the given ID and returns True if successful
pub async fn delete_place(configuration: &configuration::Configuration, place_uuid: &str, soft_delete: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<DeletePlaceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/places/{place_uuid}", local_var_configuration.base_path, place_uuid=crate::apis::urlencode(place_uuid));
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
        let local_var_entity: Option<DeletePlaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn forward_geocoding_places_geocoding_forward_get(configuration: &configuration::Configuration, searching_place: &str, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<ForwardGeocodingPlacesGeocodingForwardGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/geocoding/forward", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("searching_place", &searching_place.to_string())]);
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
        let local_var_entity: Option<ForwardGeocodingPlacesGeocodingForwardGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets the place information given the Place ID
pub async fn get_place(configuration: &configuration::Configuration, place_uuid: &str, with_meta: Option<bool>, with_catalog: Option<bool>, with_reviews: Option<bool>, with_schedule: Option<bool>, with_collection: Option<bool>, with_blog: Option<bool>, with_tags: Option<bool>, with_categories: Option<bool>, with_company: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::PlaceResponse, Error<GetPlaceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/places/{place_uuid}", local_var_configuration.base_path, place_uuid=crate::apis::urlencode(place_uuid));
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
    if let Some(ref local_var_str) = with_collection {
        local_var_req_builder = local_var_req_builder.query(&[("with_collection", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = with_company {
        local_var_req_builder = local_var_req_builder.query(&[("with_company", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetPlaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn reverse_geocoding_places_geocoding_reverse_get(configuration: &configuration::Configuration, long: f32, lat: f32, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<serde_json::Value, Error<ReverseGeocodingPlacesGeocodingReverseGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/geocoding/reverse", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("long", &long.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("lat", &lat.to_string())]);
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
        let local_var_entity: Option<ReverseGeocodingPlacesGeocodingReverseGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search all places and returns paginated results with Places being stored in items field. Can search by `project_uuid, name, address, address_line_2, city, province_state, country, postal_zip_code, lat, lng email` string fields or the `is_public and is_deleted` boolean fields. To search with these fields use query params with string values. For sorting fill out \"sort_desc\" field with either true/false and the \"sort_on\" query parameter with column you want to sort on (ex: name). Max pagination items per page is 50. Item return format: ``` {     uuid                                **type:** string     project_uuid                        **type:** string or None      meta_uuid                           **type:** string or None      catalog_data                        **type:** dict or None      review_group_data                   **type:** dict or None      schedule_data                       **type:** dict or None      collection_data                     **type:** dict or None      blog_data                           **type:** dict or None      tags                                **type:** [TagBase] or None      categories                          **type:** [CategoryBase] or None      company                             **type:** CompanyBase or None      created_at                          **type:** string or None      updated_at                          **type:** string or None      deleted_at                          **type:** string or None  } ```
pub async fn search_places(configuration: &configuration::Configuration, name: Option<&str>, address_line_1: Option<&str>, address_line_2: Option<&str>, city: Option<&str>, province_state: Option<&str>, country: Option<&str>, postal_zip_code: Option<&str>, lat: Option<&str>, lng: Option<&str>, email: Option<&str>, is_public: Option<bool>, is_deleted: Option<bool>, with_company: Option<bool>, with_meta: Option<bool>, with_catalog: Option<bool>, with_reviews: Option<bool>, with_schedule: Option<bool>, with_collection: Option<bool>, with_blog: Option<bool>, with_tags: Option<bool>, with_categories: Option<bool>, page: Option<i32>, page_size: Option<i32>, sort_on: Option<&str>, sort_desc: Option<bool>, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::Page, Error<SearchPlacesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/places", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = address_line_1 {
        local_var_req_builder = local_var_req_builder.query(&[("address_line_1", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = address_line_2 {
        local_var_req_builder = local_var_req_builder.query(&[("address_line_2", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = city {
        local_var_req_builder = local_var_req_builder.query(&[("city", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = province_state {
        local_var_req_builder = local_var_req_builder.query(&[("province_state", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = country {
        local_var_req_builder = local_var_req_builder.query(&[("country", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = postal_zip_code {
        local_var_req_builder = local_var_req_builder.query(&[("postal_zip_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = lat {
        local_var_req_builder = local_var_req_builder.query(&[("lat", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = lng {
        local_var_req_builder = local_var_req_builder.query(&[("lng", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = with_company {
        local_var_req_builder = local_var_req_builder.query(&[("with_company", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = with_collection {
        local_var_req_builder = local_var_req_builder.query(&[("with_collection", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SearchPlacesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update Place with given info, only updating the fields supplied. Place Uuid must be sent however.
pub async fn update_place(configuration: &configuration::Configuration, place_uuid: &str, place_base: crate::models::PlaceBase, x_access_token: Option<&str>, x_secret_token: Option<&str>, authorization: Option<&str>, ehelply_active_participant: Option<&str>, ehelply_project: Option<&str>, ehelply_data: Option<&str>) -> Result<crate::models::PlaceResponse, Error<UpdatePlaceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/places/{place_uuid}", local_var_configuration.base_path, place_uuid=crate::apis::urlencode(place_uuid));
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
    local_var_req_builder = local_var_req_builder.json(&place_base);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePlaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


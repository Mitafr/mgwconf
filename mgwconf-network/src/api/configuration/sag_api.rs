/*
 * Microgateway configuration API
 *
 * # Introduction This is the configuration API for Swift Microgateway (MGW). It allows remotly configure Microgateway. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway configuration API and how to integrate the use of the MGW within their organisation   * Software developers using the API to configure MGW
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::api::configuration::ResponseContent;

/// struct for typed errors of method [`sag_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SagCreateError {
    Status400(crate::model::configuration::ErrorDefaultResponse),
    Status401(),
    Status409(crate::model::configuration::ErrorCode),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sag_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SagDeleteError {
    Status400(crate::model::configuration::ErrorDefaultResponse),
    Status401(),
    Status404(crate::model::configuration::ErrorCode),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sag_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SagGetError {
    Status401(),
    Status404(),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sag_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SagUpdateError {
    Status400(crate::model::configuration::ErrorDefaultResponse),
    Status401(),
    Status404(crate::model::configuration::ErrorCode),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// This API is to create new SAG configuration in Swift Microgateway persistence storage
pub async fn sag_create(
    configuration: &configuration::Configuration,
    sag_entity: crate::model::configuration::SagEntity,
) -> Result<(), Error<SagCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sag", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&sag_entity);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SagCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This API is to delete existing SAG configuration in Swift Microgateway persistence storage
pub async fn sag_delete(
    configuration: &configuration::Configuration,
    hostname: &str,
    port: i32,
) -> Result<(), Error<SagDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sag", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("hostname", &hostname.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("port", &port.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SagDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This API is to get SAG configuration(s) on Swift Microgateway.  Use no parameters to get all available configuration or hostname & port together to get information about some SAG
pub async fn sag_get(
    configuration: &configuration::Configuration,
    hostname: Option<&str>,
    port: Option<i32>,
) -> Result<serde_json::Value, Error<SagGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sag", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = hostname {
        local_var_req_builder =
            local_var_req_builder.query(&[("hostname", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = port {
        local_var_req_builder =
            local_var_req_builder.query(&[("port", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SagGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This API is to update existing SAG configuration in Swift Microgateway persistence storage
pub async fn sag_update(
    configuration: &configuration::Configuration,
    hostname: &str,
    port: i32,
    sag_entity: crate::model::configuration::SagEntity,
) -> Result<serde_json::Value, Error<SagUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sag", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("hostname", &hostname.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("port", &port.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-KEY", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&sag_entity);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SagUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

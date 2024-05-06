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

/// struct for typed errors of method [`forward_proxies_info_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardProxiesInfoGetError {
    Status401(),
    Status404(),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forward_proxy_info_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardProxyInfoCreateError {
    Status400(crate::model::configuration::ErrorDefaultResponse),
    Status401(),
    Status409(crate::model::configuration::ErrorCode),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forward_proxy_info_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardProxyInfoDeleteError {
    Status400(crate::model::configuration::ErrorDefaultResponse),
    Status401(),
    Status404(crate::model::configuration::ErrorCode),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forward_proxy_info_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardProxyInfoUpdateError {
    Status400(crate::model::configuration::ErrorDefaultResponse),
    Status401(),
    Status404(crate::model::configuration::ErrorCode),
    Status500(crate::model::configuration::ErrorCode),
    DefaultResponse(crate::model::configuration::ErrorCode),
    UnknownValue(serde_json::Value),
}

/// This API is to get forward proxies info for MGW.  Use no parameters or hostname & port together
pub async fn forward_proxies_info_get(
    configuration: &configuration::Configuration,
    hostname: Option<&str>,
    port: Option<&str>,
) -> Result<serde_json::Value, Error<ForwardProxiesInfoGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forward-proxy", local_var_configuration.base_path);
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
        let local_var_entity: Option<ForwardProxiesInfoGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This API is to save forward proxy info  in Swift Microgateway persistence storage
pub async fn forward_proxy_info_create(
    configuration: &configuration::Configuration,
    forward_proxy_entity: crate::model::configuration::ForwardProxyEntity,
) -> Result<(), Error<ForwardProxyInfoCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forward-proxy", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&forward_proxy_entity);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ForwardProxyInfoCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This API is to delete existing forward proxy info.
pub async fn forward_proxy_info_delete(
    configuration: &configuration::Configuration,
    hostname: &str,
    port: &str,
) -> Result<(), Error<ForwardProxyInfoDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forward-proxy", local_var_configuration.base_path);
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
        let local_var_entity: Option<ForwardProxyInfoDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This API is to update forward proxy info in Swift Microgateway persistence storage
pub async fn forward_proxy_info_update(
    configuration: &configuration::Configuration,
    hostname: &str,
    port: &str,
    forward_proxy_entity: crate::model::configuration::ForwardProxyEntity,
) -> Result<serde_json::Value, Error<ForwardProxyInfoUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forward-proxy", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&forward_proxy_entity);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForwardProxyInfoUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/*
 * Microgateway management API
 *
 * # Introduction This is the management API for Swift Microgateway (MGW). It allows you remotely manage and control a bunch of Microgateway services API as SAG, etc. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway management API and how to integrate the use of the MGW within their organisation   * Software developers/SAG managers using the API to manage MGW service's
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::api::management::ResponseContent;

/// struct for typed errors of method [`verify_jars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyJarsError {
    Status400(crate::model::management::ErrorCode),
    Status401(),
    Status500(crate::model::management::ErrorCode),
    DefaultResponse(crate::model::management::ErrorDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// This API is to perform the verification of jars signature
pub async fn verify_jars(
    configuration: &configuration::Configuration,
    path: Option<&str>,
) -> Result<Vec<crate::model::management::JarVerificationEntity>, Error<VerifyJarsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/verify/jars", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = path {
        local_var_req_builder =
            local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
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
        let local_var_entity: Option<VerifyJarsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

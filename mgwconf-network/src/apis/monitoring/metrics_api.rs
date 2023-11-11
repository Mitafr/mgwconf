/*
 * Microgateway monitoring API
 *
 * # Introduction This is the monitoring API for Swift Microgateway (MGW). It allows you remotly manage and control a bunch of Microgateway services API as SAG, etc. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway management API and how to integrate the use of the MGW within their organisation   * Software developers/SAG managers using the API to manage MGW service's
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error, ResponseContent};
use crate::models::monitoring::{ErrorDefaultResponse, MetricsList};

/// struct for typed errors of method [`specific_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpecificMetricError {
    Status400(ErrorDefaultResponse),
    Status403(ErrorDefaultResponse),
    Status500(ErrorDefaultResponse),
    DefaultResponse(ErrorDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// This API is to get information by specific metric. Full list is availble by 'metrics' service.
pub async fn specific_metric(configuration: &configuration::Configuration, required_metric_name: MetricsList) -> Result<String, Error<SpecificMetricError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metrics/{requiredMetricName}", local_var_configuration.base_path, requiredMetricName = required_metric_name.to_string());
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<SpecificMetricError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/*
 * Microgateway configuration API
 *
 * # Introduction This is the configuration API for Swift Microgateway (MGW). It allows remotly configure Microgateway. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway configuration API and how to integrate the use of the MGW within their organisation   * Software developers using the API to configure MGW
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

use std::any::Any;

use crate::model::InnerEntityTrait;

/// ForwardProxyEntity : Forward proxy information: host and port, optional user and password

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardProxyEntity {
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ForwardProxyEntity {
    /// Forward proxy information: host and port, optional user and password
    pub fn new(hostname: String, port: i32) -> ForwardProxyEntity {
        ForwardProxyEntity {
            hostname,
            port,
            user: None,
            password: None,
        }
    }
}

impl InnerEntityTrait for ForwardProxyEntity {
    fn entity_type(&self) -> &str {
        "Forward Proxy"
    }
    fn name(&self) -> &str {
        &self.hostname
    }
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

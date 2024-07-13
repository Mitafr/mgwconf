/*
 * Microgateway configuration API
 *
 * # Introduction This is the configuration API for Swift Microgateway (MGW). It allows remotly configure Microgateway. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway configuration API and how to integrate the use of the MGW within their organisation   * Software developers using the API to configure MGW 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use crate::InnerEntityTrait;

/// BusinessApplicationEntity : Business application information: application name, shared secret
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessApplicationEntity {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "sharedSecret", skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
}

impl BusinessApplicationEntity {
    /// Business application information: application name, shared secret
    pub fn new(application_name: String) -> BusinessApplicationEntity {
        BusinessApplicationEntity {
            application_name,
            shared_secret: None,
        }
    }
}

impl InnerEntityTrait for BusinessApplicationEntity {
    fn entity_type(&self) -> &str {
        "BusinessApplicationEntity"
    }
    fn name(&self) -> String {
        self.application_name.to_string()
    }
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}



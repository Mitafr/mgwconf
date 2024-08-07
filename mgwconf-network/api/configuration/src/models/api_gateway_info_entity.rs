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

/// ApiGatewayInfoEntity : Api Gateway Info with environment and public certificate alias
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiGatewayInfoEntity {
    #[serde(rename = "environment")]
    pub environment: models::Environment,
    #[serde(rename = "publicCertAlias")]
    pub public_cert_alias: String,
}

impl ApiGatewayInfoEntity {
    /// Api Gateway Info with environment and public certificate alias
    pub fn new(
        environment: models::Environment,
        public_cert_alias: String,
    ) -> ApiGatewayInfoEntity {
        ApiGatewayInfoEntity {
            environment,
            public_cert_alias,
        }
    }
}

impl InnerEntityTrait for ApiGatewayInfoEntity {
    fn entity_type(&self) -> &str {
        "ApiGatewayInfoEntity"
    }
    fn name(&self) -> String {
        self.environment.to_string()
    }
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

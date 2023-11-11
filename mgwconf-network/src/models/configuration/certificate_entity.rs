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

use crate::models::InnerEntityTrait;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateEntity {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "certificateX509")]
    pub certificate_x509: String,
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

impl CertificateEntity {
    pub fn new(alias: String, certificate_x509: String) -> CertificateEntity {
        CertificateEntity {
            alias,
            certificate_x509,
            private_key: None,
        }
    }
}

impl InnerEntityTrait for CertificateEntity {
    fn name(&self) -> &str {
        &self.alias
    }
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

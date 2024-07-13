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

/// SagEntity : SAG information to create: host, port, userDN(s), messagePartner, lauKey, sslDN, active/not active, alias of public certificate for 1-way TLS
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SagEntity {
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "messagePartnerName", skip_serializing_if = "Option::is_none")]
    pub message_partner_name: Option<String>,
    #[serde(rename = "userDNs")]
    pub user_dns: Vec<String>,
    #[serde(rename = "lauKey", skip_serializing_if = "Option::is_none")]
    pub lau_key: Option<String>,
    #[serde(rename = "sslDN", skip_serializing_if = "Option::is_none")]
    pub ssl_dn: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        rename = "publicCertificateAlias",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_certificate_alias: Option<String>,
}

impl SagEntity {
    /// SAG information to create: host, port, userDN(s), messagePartner, lauKey, sslDN, active/not active, alias of public certificate for 1-way TLS
    pub fn new(hostname: String, port: i32, user_dns: Vec<String>) -> SagEntity {
        SagEntity {
            hostname,
            port,
            message_partner_name: None,
            user_dns,
            lau_key: None,
            ssl_dn: None,
            active: None,
            public_certificate_alias: None,
        }
    }
}

impl InnerEntityTrait for SagEntity {
    fn entity_type(&self) -> &str {
        "SagEntity"
    }
    fn name(&self) -> String {
        self.hostname.to_string()
    }
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

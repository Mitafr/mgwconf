/*
 * Microgateway configuration API
 *
 * # Introduction This is the configuration API for Swift Microgateway (MGW). It allows remotly configure Microgateway. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway configuration API and how to integrate the use of the MGW within their organisation   * Software developers using the API to configure MGW
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorCode {
    #[serde(rename = "error_code")]
    pub error_code: ErrorCodeEnum,
    #[serde(rename = "error_description")]
    pub error_description: String,
}

impl ErrorCode {
    pub fn new(error_code: ErrorCodeEnum, error_description: String) -> ErrorCode {
        ErrorCode { error_code, error_description }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCodeEnum {
    #[serde(rename = "MGW")]
    Mgw,
    #[serde(rename = "MGW_Configuration")]
    MgwConfiguration,
}

impl Default for ErrorCodeEnum {
    fn default() -> ErrorCodeEnum {
        Self::Mgw
    }
}
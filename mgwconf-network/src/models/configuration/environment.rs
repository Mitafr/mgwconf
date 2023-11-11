/*
 * Microgateway configuration API
 *
 * # Introduction This is the configuration API for Swift Microgateway (MGW). It allows remotly configure Microgateway. # Authentication Use API Key shared between Business Application and Microgateway. # Audience:    * Business analysts and architects to understand the functionality of the Microgateway configuration API and how to integrate the use of the MGW within their organisation   * Software developers using the API to configure MGW
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developer@swift.com
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "DEV")]
    Dev,
    #[serde(rename = "SANDBOX")]
    Sandbox,
    #[serde(rename = "TEST")]
    Test,
    #[serde(rename = "LIVE")]
    Live,
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Self::Dev => String::from("DEV"),
            Self::Sandbox => String::from("SANDBOX"),
            Self::Test => String::from("TEST"),
            Self::Live => String::from("LIVE"),
        }
    }
}

impl Default for Environment {
    fn default() -> Environment {
        Self::Dev
    }
}

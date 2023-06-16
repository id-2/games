/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerCaptchaHcaptcha : hCpatcha configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerCaptchaHcaptcha {
    #[serde(rename = "level")]
    pub level: crate::models::CloudVersionMatchmakerCaptchaHcaptchaLevel,
}

impl CloudVersionMatchmakerCaptchaHcaptcha {
    /// hCpatcha configuration.
    pub fn new(level: crate::models::CloudVersionMatchmakerCaptchaHcaptchaLevel) -> CloudVersionMatchmakerCaptchaHcaptcha {
        CloudVersionMatchmakerCaptchaHcaptcha {
            level,
        }
    }
}



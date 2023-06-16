/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesPrepareCustomAvatarUploadRequest {
    /// Unsigned 64 bit integer.
    #[serde(rename = "content_length", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// The MIME type of the custom avatar.
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    /// The path/filename of the custom avatar.
    #[serde(rename = "path")]
    pub path: String,
}

impl CloudGamesPrepareCustomAvatarUploadRequest {
    pub fn new(path: String) -> CloudGamesPrepareCustomAvatarUploadRequest {
        CloudGamesPrepareCustomAvatarUploadRequest {
            content_length: None,
            mime: None,
            path,
        }
    }
}



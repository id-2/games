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
pub struct ChatSetThreadReadRequest {
    /// Any messages newer than this timestamp will be marked as unread. This should be the current timestamp (in milliseconds).
    #[serde(rename = "last_read_ts")]
    pub last_read_ts: String,
}

impl ChatSetThreadReadRequest {
    pub fn new(last_read_ts: String) -> ChatSetThreadReadRequest {
        ChatSetThreadReadRequest {
            last_read_ts,
        }
    }
}



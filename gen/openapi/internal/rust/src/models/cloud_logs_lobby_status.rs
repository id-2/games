/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudLogsLobbyStatus : A union representing the state of a lobby.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudLogsLobbyStatus {
    #[serde(rename = "running")]
    pub running: serde_json::Value,
    #[serde(rename = "stopped", skip_serializing_if = "Option::is_none")]
    pub stopped: Option<Box<crate::models::CloudLogsLobbyStatusStopped>>,
}

impl CloudLogsLobbyStatus {
    /// A union representing the state of a lobby.
    pub fn new(running: serde_json::Value) -> CloudLogsLobbyStatus {
        CloudLogsLobbyStatus {
            running,
            stopped: None,
        }
    }
}



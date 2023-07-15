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
pub struct MatchmakerLobbiesCreateRequest {
    #[serde(rename = "lobby_config", deserialize_with = "Option::deserialize")]
    pub lobby_config: Option<serde_json::Value>,
    #[serde(rename = "publicity")]
    pub publicity: crate::models::MatchmakerCustomGamePublicity,
    #[serde(rename = "verification_data", deserialize_with = "Option::deserialize")]
    pub verification_data: Option<serde_json::Value>,
}

impl MatchmakerLobbiesCreateRequest {
    pub fn new(lobby_config: Option<serde_json::Value>, publicity: crate::models::MatchmakerCustomGamePublicity, verification_data: Option<serde_json::Value>) -> MatchmakerLobbiesCreateRequest {
        MatchmakerLobbiesCreateRequest {
            lobby_config,
            publicity,
            verification_data,
        }
    }
}



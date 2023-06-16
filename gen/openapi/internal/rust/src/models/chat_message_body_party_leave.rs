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
pub struct ChatMessageBodyPartyLeave {
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::IdentityHandle>,
}

impl ChatMessageBodyPartyLeave {
    pub fn new(identity: crate::models::IdentityHandle) -> ChatMessageBodyPartyLeave {
        ChatMessageBodyPartyLeave {
            identity: Box::new(identity),
        }
    }
}



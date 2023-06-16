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
pub struct ChatMessageBodyDeleted {
    #[serde(rename = "sender")]
    pub sender: Box<crate::models::IdentityHandle>,
}

impl ChatMessageBodyDeleted {
    pub fn new(sender: crate::models::IdentityHandle) -> ChatMessageBodyDeleted {
        ChatMessageBodyDeleted {
            sender: Box::new(sender),
        }
    }
}



use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Connection {
    #[serde(rename = "type")]
    pub connection_type: String,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStatus {
    pub success: bool,
    #[serde(rename = "description")]
    pub success_description: Option<String>,
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
}

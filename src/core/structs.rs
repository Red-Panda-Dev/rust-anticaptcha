use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

use super::constants::SOFT_ID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTaskRequest {
    clientKey: String,
    task: HashMap<String, String>,
    softId: String,
    callbackUrl: String,
}
impl CreateTaskRequest {
    pub fn new(clientKey: String, task: HashMap<String, String>, callbackUrl: String) -> Self {
        CreateTaskRequest {
            clientKey,
            task,
            callbackUrl,
            softId: SOFT_ID.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultTaskRequest {
    pub clientKey: String,
    pub taskId: String,
}

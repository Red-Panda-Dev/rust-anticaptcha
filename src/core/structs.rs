use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::constants::SOFT_ID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTaskRequest {
    clientKey: String,
    task: Value,
    softId: String,
    callbackUrl: String,
}
impl CreateTaskRequest {
    pub fn new(clientKey: String, task: Value, callbackUrl: String) -> Self {
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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

use super::constants::SOFT_ID;
use super::enums::TaskType;

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
pub struct CreateTaskResponse {
    errorId: u16,
    errorCode: Option<String>,
    errorDescription: Option<String>,
    taskId: Option<u128>,
}

pub struct TaskPayload {
    pub r#type: TaskType,
}

impl TaskPayload {
    pub fn repr(&self) -> String {
        format!("< TaskPayload type={} >", &self.r#type.value_as_string())
    }
}

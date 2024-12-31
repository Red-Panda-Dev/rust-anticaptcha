use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::core::constants::SOFT_ID;
use crate::core::enums::TaskType;

#[derive(Serialize, Deserialize)]
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
            softId: SOFT_ID.to_string(),
            callbackUrl,
        }
    }
    pub fn into_json(self) -> String {
        // method transform create task structure fields into json
        let json_string = match serde_json::to_string(&self) {
            Ok(json_string) => json_string,
            Err(error) => panic!("Error serializing CreateTaskRequest to JSON: {}", error),
        };
        json_string
    }
}

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
        String::from("< TaskPayload ".to_string() + "type=" + &self.r#type.value_as_string() + " >")
    }
}

use std::collections::HashMap;

use crate::core::constants::SOFT_ID;
use crate::core::enums::TaskType;

pub struct CreateTaskRequest {
    clientKey: String,
    task: HashMap<String, String>,
    softId: String,
    callbackUrl: Option<String>,
}
impl CreateTaskRequest {
    pub fn new(
        clientKey: String,
        task: HashMap<String, String>,
        callbackUrl: Option<String>,
    ) -> Self {
        CreateTaskRequest {
            clientKey,
            task,
            softId: SOFT_ID.to_string(),
            callbackUrl,
        }
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

use std::collections::HashMap;

use crate::core::enums::TaskType;

pub struct CreateTaskRequest {
    clientKey: String,
    task: HashMap<String, (String, u32)>,
    softId: u8,
    callbackUrl: Option<String>,
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

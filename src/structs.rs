use super::enums::TaskType;
use std::collections::HashMap;

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
    r#type: TaskType,
}

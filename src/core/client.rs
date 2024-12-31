use std::collections::HashMap;

use crate::core::enums::TaskType;
use crate::core::structs::CreateTaskRequest;

pub struct Client {
    pub sleep_time: u8,
    pub api_key: String,

    pub callbackUrl: Option<String>,

    task_payload: HashMap<String, String>,
}
impl Client {
    pub fn new(sleep_time: u8, api_key: String) -> Self {
        Client {
            sleep_time,
            api_key,
            callbackUrl: Some(String::new()),
            task_payload: HashMap::new(),
        }
    }
    pub fn set_callback_url(&mut self, callbackUrl: &str) {
        // method set new callback URL for client
        self.callbackUrl = Some(callbackUrl.to_string());
    }

    pub fn solve_captcha(
        &mut self,
        captcha_type: TaskType,
        captcha_params: HashMap<String, String>,
    ) {
        // method starts processing captcha

        // fill task payload with params
        self.task_payload = captcha_params;
        self.task_payload
            .insert("type".to_string(), captcha_type.value_as_string());

        self.create_task();
    }
    fn create_task(&self) {
        // method create task for captcha processing

        let create_task_payload = CreateTaskRequest::new(
            self.api_key.clone(),
            self.task_payload.clone(),
            self.callbackUrl.clone(),
        );
    }

    fn get_task_result(&self, captcha_type: &str) {}
}

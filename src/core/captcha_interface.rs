use std::collections::HashMap;

use super::enums::TaskType;
use super::request_interface::RequestInterface;
use super::structs::CreateTaskRequest;

pub struct CaptchaInterface {
    pub api_key: String,

    callbackUrl: String,
    sleep_time: u8,

    task_payload: HashMap<String, String>,
    request_interface: RequestInterface,
}
impl CaptchaInterface {
    pub fn new(api_key: String) -> Self {
        CaptchaInterface {
            api_key,
            sleep_time: 10,
            callbackUrl: String::new(),
            task_payload: HashMap::new(),
            request_interface: RequestInterface::new(),
        }
    }
    pub fn set_sleep_time(&mut self, sleep_time: u8) {
        // method set new sleep time for client
        self.sleep_time = sleep_time;
    }

    pub fn set_callback_url(&mut self, callbackUrl: &str) {
        // method set new callback URL for client
        self.callbackUrl = callbackUrl.to_string();
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
        println!("{}", create_task_payload.into_json())
    }

    fn get_task_result(&self, captcha_type: &str) {}
}

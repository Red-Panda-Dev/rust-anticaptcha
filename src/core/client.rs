use std::collections::HashMap;

use crate::core::constants::BASE_REQUEST_URL;
use crate::core::enums::{ControlEnpPostfix, TaskType};
use crate::core::structs::CreateTaskRequest;

pub struct Client {
    pub api_key: String,

    callbackUrl: String,
    sleep_time: u8,

    task_payload: HashMap<String, String>,
    request_client: reqwest::Client,
}
impl Client {
    pub fn new(api_key: String) -> Self {
        Client {
            api_key,
            sleep_time: 10,
            callbackUrl: String::new(),
            task_payload: HashMap::new(),
            request_client: reqwest::Client::new(),
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

    pub async fn send_post_request(
        &self,
        payload: &HashMap<String, String>,
        enp_postfix: &ControlEnpPostfix,
    ) -> serde_json::Value {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.value_as_string();

        let response = self
            .request_client
            .post(req_url)
            .json(payload)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            panic!(
                "Invalid request to API, response - {}",
                response.text().await.unwrap()
            )
        } else {
            response.json().await.unwrap()
        }
    }

    fn get_task_result(&self, captcha_type: &str) {}
}

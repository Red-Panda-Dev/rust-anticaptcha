use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use super::enums::{EnpPostfix, GetResultStatus, TaskType};
use super::request_interface::RequestInterface;
use super::structs::{CreateTaskRequest, ResultTaskRequest};

pub struct CaptchaInterface {
    pub api_key: String,

    callbackUrl: String,
    sleep_time: u8,
    max_attempts: u8,

    task_payload: HashMap<String, String>,
    request_interface: RequestInterface,
}
impl CaptchaInterface {
    pub fn new(api_key: String) -> Self {
        CaptchaInterface {
            api_key,
            sleep_time: 10,
            max_attempts: 5,
            callbackUrl: String::new(),
            task_payload: HashMap::new(),
            request_interface: RequestInterface::new(),
        }
    }
    pub fn set_sleep_time(&mut self, sleep_time: u8) {
        // method set new sleep time for client
        self.sleep_time = sleep_time;
    }
    pub fn set_max_attempts(&mut self, max_attempts: u8) {
        // method set new max_attempts for client
        self.max_attempts = max_attempts;
    }

    pub fn set_callback_url(&mut self, callbackUrl: &str) {
        // method set new callback URL for client
        self.callbackUrl = callbackUrl.to_string();
    }

    pub async fn solve_captcha(
        &mut self,
        captcha_type: TaskType,
        captcha_properties: HashMap<String, String>,
    ) {
        // method starts processing captcha

        // fill task payload with params
        self.task_payload = captcha_properties;
        self.task_payload
            .insert("type".to_string(), captcha_type.value_as_string());

        let task_id = self.create_task().await;

        sleep(Duration::from_secs(self.sleep_time as u64)).await;

        self.get_task_result(&task_id).await;
    }
    async fn create_task(&self) -> String {
        /// Method create task for captcha processing
        let create_task_payload = CreateTaskRequest::new(
            self.api_key.clone(),
            self.task_payload.clone(),
            self.callbackUrl.clone(),
        );

        let task_result = self
            .request_interface
            .send_create_task_request(&create_task_payload, &EnpPostfix::createTask)
            .await;

        if task_result["errorId"] == 0 {
            println!(
                "Task success created! Task id is - {}",
                task_result["taskId"]
            );
            task_result["taskId"].to_string()
        } else {
            panic!("Task is not created, error response body - {task_result}")
        }
    }

    async fn get_task_result(&self, task_id: &String) -> Value {
        /// Method wait and get task result
        let mut attempt: u8 = 0;

        let get_result_payload = ResultTaskRequest {
            clientKey: self.api_key.clone(),
            taskId: task_id.clone(),
        };

        loop {
            attempt += 1;
            let task_result = self
                .request_interface
                .send_get_result_request(&get_result_payload, &EnpPostfix::getTaskResult)
                .await;

            println!("Task creation result - {:?}", task_result);

            if task_result["errorId"] == 0 {
                println!("Task is correct processing - {}", task_result);
                if task_result["status"] == GetResultStatus::ready.value_as_string() {
                    println!("Task is finished - {}", task_result);
                    return task_result;
                }
                println!("Captcha is not ready ...");
            } else {
                println!("Task is not created, error response body - {task_result}");
                return task_result;
            }

            if attempt > self.max_attempts {
                return task_result;
            }

            println!("Sleeeeeping");
            sleep(Duration::from_secs(self.sleep_time as u64)).await;
        }
    }
}

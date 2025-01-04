use std::time::Duration;

use serde_json::{json, Value};
use tokio::time::sleep;

use super::enums::{EnpPostfix, GetResultStatus, TaskTypeTrait};
use super::request_interface::RequestInterface;
use super::structs::{CreateTaskRequest, ResultTaskRequest};

pub struct CaptchaInterface {
    pub api_key: String, // service API key

    callbackUrl: String, // optional web address where we can send the results of
    // captcha task processing
    sleep_time: u8,   // sleep time between requests for task result receive
    max_attempts: u8, // number of max retries for task result receive

    task_payload: Value,
    request_interface: RequestInterface,
}
impl CaptchaInterface {
    pub fn new(api_key: String) -> Self {
        CaptchaInterface {
            api_key,
            sleep_time: 10,
            max_attempts: 5,
            callbackUrl: String::new(),
            task_payload: json!({}),
            request_interface: RequestInterface::new(),
        }
    }

    /// Method set new sleep time for client
    ///
    /// # Examples
    ///
    /// ```
    /// let mut image_to_text_client = ImageCaptcha::new(API_KEY);
    /// image_to_text_client.captcha_interface.set_sleep_time(3);
    /// ```
    ///
    pub fn set_sleep_time(&mut self, sleep_time: u8) {
        self.sleep_time = sleep_time;
    }

    /// Method set new max_attempts for client
    ///
    /// # Examples
    ///
    /// ```
    /// let mut image_to_text_client = ImageCaptcha::new(API_KEY);
    /// image_to_text_client.captcha_interface.set_max_attempts(9);
    /// ```
    ///
    pub fn set_max_attempts(&mut self, max_attempts: u8) {
        self.max_attempts = max_attempts;
    }

    /// Method set new callback URL for client
    ///
    /// # Examples
    ///
    /// ```
    /// let mut image_to_text_client = ImageCaptcha::new(API_KEY);
    /// image_to_text_client.captcha_interface.set_callback_url("some-url".to_string());
    /// ```
    ///
    pub fn set_callback_url(&mut self, callbackUrl: &str) {
        self.callbackUrl = callbackUrl.to_string();
    }

    /// Method starts processing captcha
    ///
    /// # Arguments
    /// `captcha_type` - One of image captcha types from `TaskTypeTrait`
    /// `captcha_properties` - JSON with keys/values for `task` key in payload
    ///
    /// # Examples
    ///
    /// ```
    /// self.captcha_interface.solve_captcha(TaskType::ImageToTextTask, task_payload.clone()).await
    /// ```
    ///
    pub async fn solve_captcha<CaptchaType: TaskTypeTrait>(
        &mut self,
        captcha_type: CaptchaType,
        captcha_properties: Value,
    ) -> Value {
        // fill task payload with params
        self.task_payload = captcha_properties;
        self.task_payload["type"] = Value::String(captcha_type.value_as_string());

        let task_id = match self.create_task().await {
            Ok(task_id) => task_id,
            Err(error_response) => return error_response,
        };

        sleep(Duration::from_secs(self.sleep_time as u64)).await;

        self.get_task_result(&task_id).await
    }

    /// Method create task for captcha processing
    /// If task not created - return server response JSON value
    ///
    /// # Examples
    ///
    /// ```
    /// self.create_task().await.unwrap()
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/createTask>
    ///
    async fn create_task(&self) -> Result<String, Value> {
        let create_task_payload = CreateTaskRequest::new(
            self.api_key.clone(),
            self.task_payload.clone(),
            self.callbackUrl.clone(),
        );
        let task_result = self
            .request_interface
            .send_create_task_request(&create_task_payload, &EnpPostfix::createTask)
            .await
            .unwrap_or_else(|error| {
                json!({
                    "errorId": 1.to_string(),
                    "errorDescription": error
                })
            });

        if task_result["errorId"] == 0 {
            Ok(task_result["taskId"].to_string())
        } else {
            Err(task_result)
        }
    }

    /// Method wait and get task result
    ///
    /// # Examples
    ///
    /// ```
    /// self.get_task_result(&task_id).await
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getTaskResult>
    ///
    async fn get_task_result(&self, task_id: &String) -> Value {
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
                .await
                .unwrap_or_else(|error| {
                    json!({
                        "errorId": 1.to_string(),
                        "taskId": task_id.to_string(),
                        "errorDescription": error
                    })
                });

            if task_result["errorId"] == 0 {
                if task_result["status"] == GetResultStatus::ready.value_as_string() {
                    return task_result;
                }
            } else {
                return task_result;
            }

            if attempt > self.max_attempts {
                return task_result;
            }

            sleep(Duration::from_secs(self.sleep_time as u64)).await;
        }
    }
}

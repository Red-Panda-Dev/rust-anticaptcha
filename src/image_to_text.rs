use super::core::captcha_interface::CaptchaInterface;
use super::core::enums::{EnpPostfix, TaskType};
use super::core::request_interface::RequestInterface;
use std::collections::HashMap;

pub struct ImageToText {
    /// Captcha solving method - `ImageToTextTask `
    ///
    /// # Examples
    ///
    /// ```
    /// let image_to_text_client = ImageToText::new(API_KEY);
    /// let mut map: HashMap<String, String> = HashMap::new();
    /// map.insert("queueId".to_string(), 6.to_string());
    /// let result: serde_json::Value = control_client.get_queue_stats(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// https://anti-captcha.com/apidoc/task-types/ImageToTextTask
    captcha_interface: CaptchaInterface,
}
impl ImageToText {
    pub fn new(api_key: String) -> Self {
        /// Method init new Control struct with Web Requests Client
        ///
        /// # Examples
        ///
        /// ```
        /// let image_to_text_client = ImageToText::new(API_KEY);
        /// image_to_text_client.captcha_handler();
        /// ```
        ImageToText {
            captcha_interface: CaptchaInterface::new(api_key),
        }
    }

    pub async fn captcha_handler(&mut self, task_payload: &HashMap<String, String>) {
        self.captcha_interface
            .solve_captcha(TaskType::ImageToTextTask, task_payload.clone());
    }
}

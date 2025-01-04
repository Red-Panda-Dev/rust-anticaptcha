use std::collections::HashMap;

use super::core::captcha_interface::CaptchaInterface;
use super::core::enums::TaskType;

/// Captcha solving method - `ImageToTextTask `
///
/// # Examples
///
/// ```
/// let mut map: HashMap<String, String> = HashMap::new();
/// map.insert("body".to_string(), image_file_base64);
/// let image_to_text_client = ImageToText::new(API_KEY);
/// let result = image_to_text_client.captcha_handler(&map);
/// ```
///
/// # Notes
/// Read more here:
///
/// <https://anti-captcha.com/apidoc/task-types/ImageToTextTask>
pub struct ImageToText {
    pub captcha_interface: CaptchaInterface,
}
impl ImageToText {
    /// Method init new ImageToText struct with Captcha Interface
    ///
    /// # Examples
    ///
    /// ```
    /// let image_to_text_client = ImageToText::new(API_KEY);
    /// ```
    pub fn new(api_key: String) -> Self {
        ImageToText {
            captcha_interface: CaptchaInterface::new(api_key),
        }
    }

    /// Method run captcha solving logic
    ///
    /// # Examples
    ///
    /// ```
    /// let mut map: HashMap<String, String> = HashMap::new();
    /// map.insert("body".to_string(), image_file_base64);
    /// let image_to_text_client = ImageToText::new(API_KEY);
    /// let result = image_to_text_client.captcha_handler(&map);
    /// ```
    pub async fn captcha_handler(
        &mut self,
        task_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        self.captcha_interface
            .solve_captcha(TaskType::ImageToTextTask, task_payload.clone())
            .await
    }
}

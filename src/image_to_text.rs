use std::collections::HashMap;

use super::core::captcha_interface::CaptchaInterface;
use super::core::enums::TaskType;

/// Captcha solving method - `ImageToTextTask `
///
/// # Examples
///
/// With already prepared base64 string
/// ```
/// let mut map: HashMap<String, String> = HashMap::new();
/// map.insert("body".to_string(), base64_string);
/// let image_to_text_client = ImageToText::new(API_KEY);
/// let result = image_to_text_client.captcha_handler(&map);
/// ```
///
/// With image as file
/// ```
/// let image_instrument = ImageInstrument::new();
/// let image_file_base64 = image_instrument.read_image_file("captcha-image.jpg".to_string());
///
/// let mut map: HashMap<String, String> = HashMap::new();
/// map.insert("body".to_string(), image_file_base64);
///
/// let image_to_text_client = ImageToText::new(API_KEY);
/// let result = image_to_text_client.captcha_handler(&map);
/// ```
///
/// With image as link
/// ```
/// let image_instrument = ImageInstrument::new();
/// let image_link_base64 = image_instrument.read_image_link("https://captcha-image.jpg".to_string()).await;
///
/// let mut map: HashMap<String, String> = HashMap::new();
/// map.insert("body".to_string(), image_link_base64);
///
/// let image_to_text_client = ImageToText::new(API_KEY);
/// let result = image_to_text_client.captcha_handler(&map);
/// ```
///
/// # Notes
/// Read more here:
///
/// <https://anti-captcha.com/apidoc/task-types/ImageToTextTask>
///
pub struct ImageToText {
    pub captcha_interface: CaptchaInterface,
}
impl ImageToText {
    /// Method init new ImageToText struct with Captcha Interface
    ///
    /// # Arguments
    /// `api_key` - Service API key
    ///
    /// # Examples
    ///
    /// ```
    /// let image_to_text_client = ImageToText::new(API_KEY);
    /// ```
    /// # Returns
    /// Method return new `ImageToText` instance
    ///
    pub fn new(api_key: String) -> Self {
        ImageToText {
            captcha_interface: CaptchaInterface::new(api_key),
        }
    }

    /// Method run captcha solving logic
    ///
    /// # Arguments
    /// `api_key` - Service API key
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

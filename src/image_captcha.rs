use serde_json::Value;

use super::core::captcha_interface::CaptchaInterface;
use super::core::enums::ImageTaskType;

/// Captcha solving method - `ImageToTextTask` and `ImageToCoordinatesTask`
///
/// # Examples
///
/// With already prepared base64 string
/// ```no_run
/// use serde_json::json;
///
/// use rust_anticaptcha::core::enums::ImageTaskType;
/// use rust_anticaptcha::image_captcha::ImageCaptcha;
///
/// async fn run() {
///     let map = json!({"body": "base64_string"});
///
///     let mut image_to_text_client = ImageCaptcha::new("API_KEY".to_string());
///     image_to_text_client.captcha_handler(ImageTaskType::ImageToCoordinatesTask, map).await;
/// }
/// ```
///
/// With image as file
/// ```no_run
/// use serde_json::json;
///
/// use rust_anticaptcha::core::enums::ImageTaskType;
/// use rust_anticaptcha::image_captcha::ImageCaptcha;
/// use rust_anticaptcha::instruments::image_instrument::ImageInstrument;
///
/// async fn run() {
///     let image_instrument = ImageInstrument::new();
///     let image_file_base64 = image_instrument.read_image_file("files/captcha-image.jpg".to_string());
///
///     let map = json!({"body": image_file_base64});
///
///     let mut image_to_text_client = ImageCaptcha::new("API_KEY".to_string());
///     image_to_text_client.captcha_handler(ImageTaskType::ImageToTextTask, map).await;
/// }
/// ```
///
/// With image as link
/// ```no_run
/// use serde_json::json;
///
/// use rust_anticaptcha::core::enums::ImageTaskType;
/// use rust_anticaptcha::image_captcha::ImageCaptcha;
/// use rust_anticaptcha::instruments::image_instrument::ImageInstrument;
///
/// async fn run() {
///     let image_instrument = ImageInstrument::new();
///     let image_link_base64 = image_instrument.read_image_link("https://captcha-image.jpg".to_string()).await;
///
///     let map = json!({"body": image_link_base64});
///
///     let mut image_to_text_client = ImageCaptcha::new("API_KEY".to_string());
///     image_to_text_client.captcha_handler(ImageTaskType::ImageToTextTask, map).await;
/// }
/// ```
///
/// # Notes
/// Read more here:
///
/// <https://anti-captcha.com/apidoc/task-types/ImageToTextTask>
/// <https://anti-captcha.com/apidoc/task-types/ImageToCoordinatesTask>
///
pub struct ImageCaptcha {
    pub captcha_interface: CaptchaInterface,
}
impl ImageCaptcha {
    /// Method init new ImageCaptcha struct with Captcha Interface
    ///
    /// # Arguments
    /// `api_key` - Service API key
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_anticaptcha::image_captcha::ImageCaptcha;
    ///
    /// let image_to_text_client = ImageCaptcha::new("API_KEY".to_string());
    /// ```
    /// # Returns
    /// Method return new `ImageCaptcha` instance
    ///
    pub fn new(api_key: String) -> Self {
        ImageCaptcha {
            captcha_interface: CaptchaInterface::new(api_key),
        }
    }

    /// Method run captcha solving logic
    ///
    /// # Arguments
    /// `captcha_type` - One of image captcha types from `ImageTaskType`
    /// `task_payload` - JSON with captcha task payload
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::core::enums::ImageTaskType;
    /// use rust_anticaptcha::image_captcha::ImageCaptcha;
    ///
    /// async fn run() {
    ///     let map = json!({"body": "base64_string"});
    ///
    ///     let mut image_to_text_client = ImageCaptcha::new("API_KEY".to_string());
    ///     image_to_text_client.captcha_handler(ImageTaskType::ImageToTextTask, map).await;
    /// }
    /// ```
    pub async fn captcha_handler(
        &mut self,
        captcha_type: ImageTaskType,
        task_payload: Value,
    ) -> Value {
        self.captcha_interface
            .solve_captcha(captcha_type, task_payload)
            .await
    }
}

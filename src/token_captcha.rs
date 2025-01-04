use serde_json::Value;

use super::core::captcha_interface::CaptchaInterface;
use super::core::enums::TokenTaskType;

/// Token captcha solving method - ReCaptchaV2, ReCaptchaV3, FunCaptcha, GeeTest, Turnstile.
///
/// # Examples
/// For `RecaptchaV2TaskProxyless`.
/// Other captcha types works same, you need check docs and set correct `task_payload`.
/// ```
/// let map = json!({
///         "websiteKey": "6LfD3PIbAAAAAJs_eEHvoOl75_83eXSqpPSRFJ_u",
///         "websiteURL":"https://rucaptcha.com/demo/recaptcha-v2"
///     });
///
/// let image_to_text_client = TokenCaptcha::new(API_KEY);
/// let result = image_to_text_client.captcha_handler(TokenTaskType::RecaptchaV2TaskProxyless, map);
/// ```
///
/// # Notes
/// Read more here:
///
/// <https://anti-captcha.com/apidoc/task-types/RecaptchaV2TaskProxyless>
/// <https://anti-captcha.com/apidoc/task-types/RecaptchaV3TaskProxyless>
/// <https://anti-captcha.com/apidoc/task-types/RecaptchaV2EnterpriseTaskProxyless>
/// <https://anti-captcha.com/apidoc/task-types/RecaptchaV3Enterprise>
/// <https://anti-captcha.com/apidoc/task-types/FunCaptchaTaskProxyless>
/// <https://anti-captcha.com/apidoc/task-types/GeeTestTaskProxyless>
/// <https://anti-captcha.com/apidoc/task-types/TurnstileTaskProxyless>
/// <https://anti-captcha.com/apidoc/task-types/AntiGateTask>
///
pub struct TokenCaptcha {
    pub captcha_interface: CaptchaInterface,
}
impl TokenCaptcha {
    /// Method init new TokenCaptcha struct with Captcha Interface
    ///
    /// # Arguments
    /// `api_key` - Service API key
    ///
    /// # Examples
    ///
    /// ```
    /// let image_to_text_client = TokenCaptcha::new(API_KEY);
    /// ```
    /// # Returns
    /// Method return new `TokenCaptcha` instance
    ///
    pub fn new(api_key: String) -> Self {
        TokenCaptcha {
            captcha_interface: CaptchaInterface::new(api_key),
        }
    }

    /// Method run captcha solving logic
    ///
    /// # Arguments
    /// `captcha_type` - One of image captcha types from `TokenTaskType`
    /// `task_payload` - JSON with captcha task payload
    ///
    /// # Examples
    ///
    /// ```
    /// let map = json!({"body": base64_string});
    ///
    /// let image_to_text_client = TokenCaptcha::new(API_KEY);
    /// let result = image_to_text_client.captcha_handler(map);
    /// ```
    pub async fn captcha_handler(
        &mut self,
        captcha_type: TokenTaskType,
        task_payload: Value,
    ) -> Value {
        self.captcha_interface
            .solve_captcha(captcha_type, task_payload)
            .await
    }
}

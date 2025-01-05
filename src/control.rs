//! # Control module
//!
//! This module is responsible for additional AntiCaptcha API service methods:
//!
//! - Get balance
//! - Get app statistics
//! - Report incorrect captcha
//! - Report correct captcha
//! - Get spending stats
//! - Push antigate variables
//!
//! ## Basic example for Control `get_balance` method
//!
//! ```no_run
//! use rust_anticaptcha::control::Control;
//!
//! async fn run() {
//!     let control_client = Control::new();
//!     control_client.get_balance(&"API_KEY".to_string()).await;
//! }
//! ```
//!
//! # Notes
//! Read more here:
//!
//! <https://anti-captcha.com/apidoc/methods/getBalance>
//!
//! <https://anti-captcha.com/apidoc/methods/getAppStats>
//!
//! <https://anti-captcha.com/apidoc/methods/getSpendingStats>
//!
use serde_json::{json, Value};

use super::core::constants::BASE_REQUEST_URL;
use super::core::enums::EnpPostfix;
use super::core::request_interface::RequestInterface;

/// Structure help processing additional AntiCaptcha methods
///
/// # Examples
///
/// ```
/// use rust_anticaptcha::control::Control;
///
/// let control_client = Control::new();
/// ```
///
/// # Notes
/// Read more here:
///
/// <https://anti-captcha.com/apidoc>
///
pub struct Control {
    request_interface: RequestInterface,
}
impl Control {
    /// Method init new Control struct with Web Requests Client
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_anticaptcha::control::Control;
    ///
    /// let control_client = Control::new();
    /// ```
    pub fn new() -> Self {
        Control {
            request_interface: RequestInterface::new(),
        }
    }

    /// Method prepare and send control request to API
    ///
    /// # Arguments
    /// `payload` - request JSON payload
    /// `enp_postfix` - API URL postfix from `EnpPostfix`
    ///
    /// # Returns
    /// Method return API response JSON
    ///
    async fn send_control_request(&self, payload: &Value, enp_postfix: &EnpPostfix) -> Value {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.to_string();
        let result = self
            .request_interface
            .send_post_request(&payload, &req_url)
            .await
            .unwrap();

        if result.status().eq(&200) {
            result.json().await.unwrap()
        } else {
            panic!(
                "{}",
                format!(
                    "Invalid request to API, status code - {} response - {}",
                    result.status(),
                    result.text().await.unwrap()
                )
            )
        }
    }

    /// Method request `getBalance` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     control_client.get_balance(&"API_KEY".to_string()).await;
    /// }
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getBalance>
    pub async fn get_balance(&self, api_key: &String) -> Value {
        let map = json!({"clientKey": api_key});

        self.send_control_request(&map, &EnpPostfix::getBalance).await
    }

    /// Method request `getQueueStats ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({"queueId": 6});
    ///     let result = control_client.get_queue_stats(&map).await;
    /// }
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getQueueStats>
    pub async fn get_queue_stats(&self, enp_payload: &Value) -> Value {
        self.send_control_request(enp_payload, &EnpPostfix::getQueueStats)
            .await
    }

    /// Method request `reportIncorrectImageCaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({"clientKey": "API_KEY", "taskId": 12345});
    ///     let result = control_client.report_incorrect_image(&map).await;
    /// }
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportIncorrectImageCaptcha>
    pub async fn report_incorrect_image(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportIncorrectImageCaptcha)
            .await
    }

    /// Method request `reportIncorrectRecaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({"clientKey": "API_KEY", "taskId": 12345});
    ///     let result = control_client.report_incorrect_recaptcha(&map).await;
    /// }
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportIncorrectRecaptcha>
    pub async fn report_incorrect_recaptcha(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportIncorrectRecaptcha)
            .await
    }

    /// Method request `reportCorrectRecaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({"clientKey": "API_KEY", "taskId": 12345});
    ///     control_client.report_correct_recaptcha(&map).await;
    /// }
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportCorrectRecaptcha>
    pub async fn report_correct_recaptcha(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportCorrectRecaptcha)
            .await
    }

    /// Method request `reportIncorrectHcaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({"clientKey": "API_KEY", "taskId": 12345});
    ///     control_client.report_incorrect_hcaptcha(&map).await;
    /// };
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportIncorrectHcaptcha>
    pub async fn report_incorrect_hcaptcha(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportIncorrectHcaptcha)
            .await
    }

    /// Method request `pushAntiGateVariable ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({
    ///                     "clientKey": "API_KEY",
    ///                     "taskId": 12345,
    ///                     "name": "my_late_variable",
    ///                     "value": "some_value"
    ///                 });
    ///     control_client.push_antigate_var(&map).await;
    /// }
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/pushAntiGateVariable>
    pub async fn push_antigate_var(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::pushAntiGateVariable)
            .await
    }

    /// Method request `getSpendingStats ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({
    ///                     "clientKey": "API_KEY",
    ///                     "date": 1672185600,
    ///                     "queue": "Recaptcha Proxyless"
    ///                 });
    ///     control_client.get_spending_stats(&map).await;
    /// }
    /// ```
    ///
    /// # Returns
    /// This method grabs account spendings and task volume statistics for a 24 hour period.
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getSpendingStats>
    pub async fn get_spending_stats(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::getSpendingStats)
            .await
    }

    /// Method request `getAppStats ` enp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// use rust_anticaptcha::control::Control;
    ///
    /// async fn run() {
    ///     let control_client = Control::new();
    ///     let map = json!({
    ///                     "clientKey": "API_KEY",
    ///                     "softId": 867,
    ///                     "mode": "money"
    ///                 });
    ///     control_client.get_app_stats(&map).await;
    /// }
    /// ```
    ///
    /// # Returns
    /// This method retrieves daily statistics for your application,
    /// which you register in Developer Center.
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getAppStats>
    pub async fn get_app_stats(&self, enp_payload: &Value) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::getAppStats)
            .await
    }
}

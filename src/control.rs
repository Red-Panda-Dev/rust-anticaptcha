use std::collections::HashMap;

use serde_json::Value;

use super::core::constants::BASE_REQUEST_URL;
use super::core::enums::EnpPostfix;
use super::core::request_interface::RequestInterface;

/// Structure help processing additional AntiCaptcha methods
///
/// # Examples
///
/// ```
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
    /// # Examples
    ///
    /// ```
    ///let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), api_key);
    /// self.send_control_request(&map, &EnpPostfix::getBalance).await
    /// ```
    ///
    /// # Returns
    /// Method return API response JSON
    ///
    async fn send_control_request(
        &self,
        payload: &HashMap<String, String>,
        enp_postfix: &EnpPostfix,
    ) -> Value {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.value_as_string();
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
    /// ```
    /// let control_client = Control::new();
    /// let result: Value = control_client.get_balance().await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getBalance>
    pub async fn get_balance(&self, api_key: String) -> Value {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("clientKey".to_string(), api_key);

        self.send_control_request(&map, &EnpPostfix::getBalance).await
    }

    /// Method request `getQueueStats ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("queueId".to_string(), 6.to_string());
    /// let result: Value = control_client.get_queue_stats(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getQueueStats>
    pub async fn get_queue_stats(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::getQueueStats)
            .await
    }

    /// Method request `reportIncorrectImageCaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("taskId".to_string(), 12345.to_string());
    /// let result: Value = control_client.report_incorrect_image(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportIncorrectImageCaptcha>
    pub async fn report_incorrect_image(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportIncorrectImageCaptcha)
            .await
    }

    /// Method request `reportIncorrectRecaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("taskId".to_string(), 12345.to_string());
    /// let result: Value = control_client.report_incorrect_recaptcha(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportIncorrectRecaptcha>
    pub async fn report_incorrect_recaptcha(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportIncorrectRecaptcha)
            .await
    }

    /// Method request `reportCorrectRecaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("taskId".to_string(), 12345.to_string());
    /// let result: Value = control_client.report_correct_recaptcha(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportCorrectRecaptcha>
    pub async fn report_correct_recaptcha(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportCorrectRecaptcha)
            .await
    }

    /// Method request `reportIncorrectHcaptcha ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("taskId".to_string(), 12345.to_string());
    /// let result: Value = control_client.report_incorrect_hcaptcha(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/reportIncorrectHcaptcha>
    pub async fn report_incorrect_hcaptcha(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::reportIncorrectHcaptcha)
            .await
    }

    /// Method request `pushAntiGateVariable ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("taskId".to_string(), 12345.to_string());
    /// map.insert("name".to_string(), "my_late_variable".to_string());
    /// map.insert("value".to_string(), "The value".to_string());
    /// let result: Value = control_client.push_antigate_var(&map).await;
    /// ```
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/pushAntiGateVariable>
    pub async fn push_antigate_var(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::pushAntiGateVariable)
            .await
    }

    /// Method request `getSpendingStats ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("date".to_string(), 1672185600.to_string());
    /// map.insert("queue".to_string(), "Recaptcha Proxyless".to_string());
    /// let result: Value = control_client.get_spending_stats(&map).await;
    /// ```
    ///
    /// # Returns
    /// This method grabs account spendings and task volume statistics for a 24 hour period.
    ///
    /// # Notes
    /// Read more here:
    ///
    /// <https://anti-captcha.com/apidoc/methods/getSpendingStats>
    pub async fn get_spending_stats(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::getSpendingStats)
            .await
    }

    /// Method request `getAppStats ` enp
    ///
    /// # Examples
    ///
    /// ```
    /// let control_client = Control::new();
    /// let mut map = HashMap::new();
    /// map.insert("clientKey".to_string(), API_KEY.to_string());
    /// map.insert("softId".to_string(), 867.to_string());
    /// map.insert("mode".to_string(), "money".to_string());
    /// let result: Value = control_client.get_app_stats(&map).await;
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
    pub async fn get_app_stats(&self, enp_payload: &HashMap<String, String>) -> Value {
        self.send_control_request(&enp_payload, &EnpPostfix::getAppStats)
            .await
    }
}

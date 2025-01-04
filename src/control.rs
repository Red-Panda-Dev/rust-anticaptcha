use std::collections::HashMap;

use super::core::enums::EnpPostfix;
use super::core::request_interface::RequestInterface;

pub struct Control {
    request_interface: RequestInterface,
}
impl Control {
    pub fn new() -> Self {
        /// Method init new Control struct with Web Requests Client
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// ```
        Control {
            request_interface: RequestInterface::new(),
        }
    }

    pub async fn get_balance(&self, api_key: String) -> serde_json::Value {
        /// Method request `getBalance` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let result: serde_json::Value = control_client.get_balance().await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/getBalance
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("clientKey".to_string(), api_key);

        self.request_interface
            .send_post_request(&map, &EnpPostfix::getBalance)
            .await
    }

    pub async fn get_queue_stats(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `getQueueStats ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("queueId".to_string(), 6.to_string());
        /// let result: serde_json::Value = control_client.get_queue_stats(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/getQueueStats
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::getQueueStats)
            .await
    }

    pub async fn report_incorrect_image(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `reportIncorrectImageCaptcha ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("taskId".to_string(), 12345.to_string());
        /// let result: serde_json::Value = control_client.report_incorrect_image(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/reportIncorrectImageCaptcha
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::reportIncorrectImageCaptcha)
            .await
    }
    pub async fn report_incorrect_recaptcha(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `reportIncorrectRecaptcha ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("taskId".to_string(), 12345.to_string());
        /// let result: serde_json::Value = control_client.report_incorrect_recaptcha(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/reportIncorrectRecaptcha
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::reportIncorrectRecaptcha)
            .await
    }

    pub async fn report_correct_recaptcha(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `reportCorrectRecaptcha ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("taskId".to_string(), 12345.to_string());
        /// let result: serde_json::Value = control_client.report_correct_recaptcha(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/reportCorrectRecaptcha
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::reportCorrectRecaptcha)
            .await
    }

    pub async fn report_incorrect_hcaptcha(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `reportIncorrectHcaptcha ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("taskId".to_string(), 12345.to_string());
        /// let result: serde_json::Value = control_client.report_incorrect_hcaptcha(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/reportIncorrectHcaptcha
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::reportIncorrectHcaptcha)
            .await
    }

    pub async fn push_antigate_var(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `pushAntiGateVariable ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("taskId".to_string(), 12345.to_string());
        /// map.insert("name".to_string(), "my_late_variable".to_string());
        /// map.insert("value".to_string(), "The value".to_string());
        /// let result: serde_json::Value = control_client.push_antigate_var(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/pushAntiGateVariable
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::pushAntiGateVariable)
            .await
    }

    pub async fn get_spending_stats(
        &self,
        enp_payload: &HashMap<String, String>,
    ) -> serde_json::Value {
        /// Method request `getSpendingStats ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("date".to_string(), 1672185600.to_string());
        /// map.insert("queue".to_string(), "Recaptcha Proxyless".to_string());
        /// let result: serde_json::Value = control_client.get_spending_stats(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/getSpendingStats
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::getSpendingStats)
            .await
    }
    pub async fn get_app_stats(&self, enp_payload: &HashMap<String, String>) -> serde_json::Value {
        /// Method request `getAppStats ` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new();
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("clientKey".to_string(), API_KEY.to_string());
        /// map.insert("softId".to_string(), 867.to_string());
        /// map.insert("mode".to_string(), "money".to_string());
        /// let result: serde_json::Value = control_client.get_app_stats(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/getAppStats
        self.request_interface
            .send_post_request(&enp_payload, &EnpPostfix::getAppStats)
            .await
    }
}

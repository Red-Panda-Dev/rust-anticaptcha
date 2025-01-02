use std::collections::HashMap;

use crate::core::client::Client as CaptchaClient;
use crate::core::enums::ControlEnpPostfix;

pub struct Control {
    captcha_client: CaptchaClient,
}
impl Control {
    pub fn new(api_key: String) -> Control {
        // method init new Control struct with Captcha Client
        Control {
            captcha_client: CaptchaClient::new(api_key),
        }
    }

    pub async fn get_balance(&self) -> serde_json::Value {
        /// Method request `getBalance` enp
        ///
        /// # Examples
        ///
        /// ```
        /// let control_client = Control::new(API_KEY);
        /// let result: serde_json::Value = control_client.get_balance().await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/getBalance
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("clientKey".to_string(), self.captcha_client.api_key.clone());

        self.captcha_client
            .send_post_request(&map, &ControlEnpPostfix::getBalance)
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
        /// let control_client = Control::new(API_KEY);
        /// let mut map: HashMap<String, String> = HashMap::new();
        /// map.insert("queueId".to_string(), 6.to_string());
        /// let result: serde_json::Value = control_client.get_queue_stats(&map).await;
        /// ```
        ///
        /// # Notes
        /// Read more here:
        ///
        /// https://anti-captcha.com/apidoc/methods/getQueueStats
        self.captcha_client
            .send_post_request(&enp_payload, &ControlEnpPostfix::getQueueStats)
            .await
    }
}

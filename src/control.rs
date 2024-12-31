use crate::core::client::Client as CaptchaClient;
use crate::core::enums::ControlEnpPostfix;
use crate::API_KEY;
use std::collections::HashMap;

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
        // Method request `getBalance` enp
        // https://anti-captcha.com/apidoc/methods/getBalance

        let map: HashMap<String, String> =
            HashMap::from(("clientKey".to_string(), API_KEY.to_string()));
        self.captcha_client
            .send_post_request(&map, &ControlEnpPostfix::getBalance)
            .await
    }
}

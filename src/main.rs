mod core;

use crate::core::enums::TaskType;
use core::client::Client;
use std::collections::HashMap;
use std::string::ToString;

const API_KEY: &str = "999999999999999999999999999";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let response = client.get("https://www.rust-lang.org").send().await?;
    println!("status code is = {}", response.status());

    let mut captcha_client = Client::new(7, API_KEY.to_string());

    let mut captcha_params: HashMap<String, String> = HashMap::new();
    captcha_params.insert(
        "websiteURL".to_string(),
        "https://docs.rs/reqwest/0.12.11/reqwest/".to_string(),
    );
    captcha_params.insert("websiteKey".to_string(), "3htd36tpbin543".to_string());
    captcha_params.insert("recaptchaDataSValue".to_string(), "".to_string());
    captcha_params.insert("isInvisible".to_string(), false.to_string());
    captcha_client.solve_captcha(TaskType::RecaptchaV2TaskProxyless, captcha_params);

    Ok(())
}

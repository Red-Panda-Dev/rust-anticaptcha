use serde_json::json;

use std::collections::HashMap;

mod control;
mod core;
mod instruments;
mod image_captcha;
mod token_captcha;

use crate::core::enums::{ImageTaskType, TokenTaskType};
use crate::image_captcha::ImageCaptcha;
use crate::instruments::image_instrument::ImageInstrument;
use crate::token_captcha::TokenCaptcha;
use control::Control;

const API_KEY: &str = "999999999999999999999999999";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get("https://www.rust-lang.org").send().await?;
    println!("status code is = {}", response.status());

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());

    let control_client = Control::new();

    let control_result: serde_json::Value = control_client.get_balance(API_KEY.to_string()).await;
    println!("Your balance - {:?}", control_result);

    let image_instrument = ImageInstrument::new();
    let image_file_base64 = image_instrument.read_image_file("files/captcha-image.jpg".to_string());

    let mut image_to_text_client = ImageCaptcha::new(API_KEY.to_string());

    let map = json!({"body": image_file_base64});
    let image_to_text_result = image_to_text_client
        .captcha_handler(ImageTaskType::ImageToTextTask, map)
        .await;
    println!("Image file to text - {:?}", image_to_text_result);

    let image_instrument = ImageInstrument::new();
    let image_link_base64 =
        image_instrument.read_image_link("https://raw.githubusercontent.com/AndreiDrang/python3-anticaptcha/refs/heads/main/files/captcha-image.jpg".to_string()).await;

    let mut image_to_text_client = ImageCaptcha::new(API_KEY.to_string());

    let map = json!({"body": image_link_base64});
    let image_to_text_result = image_to_text_client
        .captcha_handler(ImageTaskType::ImageToTextTask, map)
        .await;
    println!("Image link to text - {:?}", image_to_text_result);

    let mut token_captcha_client = TokenCaptcha::new(API_KEY.to_string());

    let map = json!({
        "websiteKey": "6LfD3PIbAAAAAJs_eEHvoOl75_83eXSqpPSRFJ_u",
        "websiteURL":"https://rucaptcha.com/demo/recaptcha-v2"
    });
    let token_captcha_result = token_captcha_client
        .captcha_handler(TokenTaskType::RecaptchaV2TaskProxyless, map)
        .await;
    println!("Token captcha result - {:?}", token_captcha_result);

    Ok(())
}

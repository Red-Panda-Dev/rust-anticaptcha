use std::collections::HashMap;

mod control;
mod core;
mod instruments;
mod image_to_text;

use crate::image_to_text::ImageToText;
use crate::instruments::image_instrument::ImageInstrument;
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
    let image_file_base64 =
        image_instrument.read_image_file("files/captcha-image-coordinates.jpg".to_string());

    let mut image_to_text_client = ImageToText::new(API_KEY.to_string());
    image_to_text_client.captcha_interface.set_sleep_time(3);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("body".to_string(), image_file_base64);
    let image_to_text_result = image_to_text_client.captcha_handler(&map).await;
    println!("Image to text - {:?}", image_to_text_result);

    Ok(())
}

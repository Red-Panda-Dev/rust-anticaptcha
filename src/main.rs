use std::collections::HashMap;

mod control;
mod core;

use crate::control::Control;

const API_KEY: &str = "999999999999999999999999999";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get("https://www.rust-lang.org").send().await?;
    println!("status code is = {}", response.status());

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());

    let control_client = Control::new(API_KEY.to_string());

    let control_result: serde_json::Value = control_client.get_balance().await;
    println!("Your balance - {:?}", control_result);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("queueId".to_string(), 6.to_string());
    let control_result: serde_json::Value = control_client.get_queue_stats(&map).await;
    println!("Queue stats - {:?}", control_result);

    Ok(())
}

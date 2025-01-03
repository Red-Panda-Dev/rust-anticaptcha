use std::collections::HashMap;

mod control;
mod core;

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

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("queueId".to_string(), 1.to_string());
    let control_result: serde_json::Value = control_client.get_queue_stats(&map).await;
    println!("Queue stats - {:?}", control_result);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());
    map.insert("taskId".to_string(), 12345.to_string());
    let control_result: serde_json::Value = control_client.report_incorrect_image(&map).await;
    println!("Report incorrect image - {:?}", control_result);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());
    map.insert("taskId".to_string(), 12345.to_string());
    let control_result: serde_json::Value = control_client.report_incorrect_recaptcha(&map).await;
    println!("Report incorrect recaptcha - {:?}", control_result);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());
    map.insert("taskId".to_string(), 12345.to_string());
    let control_result: serde_json::Value = control_client.report_correct_recaptcha(&map).await;
    println!("Report correct recaptcha - {:?}", control_result);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());
    map.insert("softId".to_string(), 867.to_string());
    let control_result: serde_json::Value = control_client.get_spending_stats(&map).await;
    println!("Spending stats - {:?}", control_result);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("clientKey".to_string(), API_KEY.to_string());
    map.insert("softId".to_string(), 867.to_string());
    map.insert("mode".to_string(), "money".to_string());
    let control_result: serde_json::Value = control_client.get_app_stats(&map).await;
    println!("App stats - {:?}", control_result);

    Ok(())
}

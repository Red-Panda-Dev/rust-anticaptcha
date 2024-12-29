mod core;
use core::enums;
use core::structs;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let response = client.get("https://www.rust-lang.org").send().await?;
    println!("status code is = {}", response.status());

    let new_task_payload = structs::TaskPayload {
        r#type: enums::TaskType::RecaptchaV2Task,
    };

    println!("Task payload - {}", new_task_payload.repr());
    Ok(())
}

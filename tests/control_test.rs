use std::collections::HashMap;

use rust_anticaptcha::control::Control;

use crate::structs::CaptchaArgs;

mod structs;

#[test]
fn control_instance() {
    Control::new();
}

#[tokio::test]
async fn success_get_balance() {
    let captcha_args = CaptchaArgs::new();

    let control_instance = Control::new();
    let result = control_instance.get_balance(captcha_args.API_KEY).await;
    assert_eq!(result["errorId"], 0);
    assert_ne!(result["balance"], 0);
}

#[tokio::test]
async fn success_get_app_stats() {
    let captcha_args = CaptchaArgs::new();

    let mut map = HashMap::new();
    map.insert("clientKey".to_string(), captcha_args.API_KEY);
    map.insert("softId".to_string(), 1220.to_string());
    map.insert("mode".to_string(), "money".to_string());

    let control_instance = Control::new();
    let result = control_instance.get_app_stats(&map).await;

    assert_eq!(result["errorId"], 0);
    assert_eq!(result["chartData"][0]["name"], "Money earned");
}

#[tokio::test]
async fn fail_get_balance() {
    let control_instance = Control::new();
    let result = control_instance.get_balance("API_KEY".to_string()).await;

    assert_eq!(result["errorId"], 1);
    assert_eq!(result["errorCode"], "ERROR_KEY_DOES_NOT_EXIST");
}

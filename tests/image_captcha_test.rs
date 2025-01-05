use serde_json::json;

use rust_anticaptcha::core::enums::{GetResultStatus, ImageTaskType};
use rust_anticaptcha::image_captcha::ImageCaptcha;
use rust_anticaptcha::instruments::image_instrument::ImageInstrument;

use crate::structs::CaptchaArgs;

mod structs;

#[test]
fn image_captcha_instance() {
    let captcha_args = CaptchaArgs::new();

    ImageCaptcha::new(captcha_args.API_KEY);
}

#[test]
fn image_instrument_instance() {
    ImageInstrument::new();
}

#[tokio::test]
async fn image_instrument_read_image_link() {
    let captcha_args = CaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_link(captcha_args.captcha_url)
        .await
        .ends_with(&captcha_args.captcha_ends_with));
}

#[test]
fn image_instrument_read_image_file() {
    let captcha_args = CaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_file(captcha_args.captcha_file)
        .ends_with(&captcha_args.captcha_ends_with));
}

#[tokio::test]
async fn image_instrument_read_coord_image_link() {
    let captcha_args = CaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_link(captcha_args.coord_captcha_url)
        .await
        .ends_with(&captcha_args.coord_captcha_ends_with));
}

#[test]
fn image_instrument_read_coord_image_file() {
    let captcha_args = CaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_file(captcha_args.coord_captcha_file)
        .ends_with(&captcha_args.coord_captcha_ends_with));
}

#[tokio::test]
async fn success_image_file() {
    let captcha_args = CaptchaArgs::new();

    let instance = ImageInstrument::new();
    let base64_str = instance.read_image_file(captcha_args.captcha_file);

    let mut solver_instance = ImageCaptcha::new(captcha_args.API_KEY);

    let result = solver_instance
        .captcha_handler(ImageTaskType::ImageToTextTask, &json!({"body":base64_str}))
        .await;
    assert_eq!(result["errorId"], 0);
    assert_eq!(result["status"], GetResultStatus::ready.to_string());
}

#[tokio::test]
async fn fail_image_file() {
    let captcha_args = CaptchaArgs::new();

    let instance = ImageInstrument::new();
    let base64_str = instance.read_image_file(captcha_args.captcha_file);

    let mut solver_instance = ImageCaptcha::new("API_KEY".to_string());

    let result = solver_instance
        .captcha_handler(ImageTaskType::ImageToTextTask, &json!({"body":base64_str}))
        .await;

    assert_eq!(result["errorId"], 1);
    assert_eq!(result["errorCode"], "ERROR_KEY_DOES_NOT_EXIST");
}

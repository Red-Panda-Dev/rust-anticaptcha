use std::any::TypeId;

use rust_anticaptcha::core::enums::TokenTaskType;
use rust_anticaptcha::image_captcha::ImageCaptcha;

use crate::structs::ImageCaptchaArgs;
use crate::structs::API_KEY;

mod structs;

struct ImageToTextTests {
    image_captcha_args: ImageCaptchaArgs,
}

impl ImageToTextTests {
    pub fn new() -> Self {
        ImageToTextTests {
            image_captcha_args: ImageCaptchaArgs::new(),
        }
    }
}

#[test]
fn it_adds_two() {
    let instance: Box<dyn std::any::Any> = Box::new(ImageCaptcha::new(API_KEY.to_string()));
    assert_eq!(TypeId::of::<ImageCaptcha>(), instance.type_id())
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env::var;

pub struct ImageCaptchaArgs {
    pub API_KEY: String,
    pub captcha_file: String,
    pub captcha_url: String,
    pub coord_captcha_file: String,
    pub coord_captcha_url: String,

    pub captcha_ends_with: String,
    pub coord_captcha_ends_with: String,
}

impl ImageCaptchaArgs {
    pub fn new() -> Self {
        ImageCaptchaArgs {
            API_KEY: var("API_KEY").expect("You must set `API_KEY` variable"),
            captcha_file: "files/captcha-image.jpg".to_string(),
            captcha_url : "https://raw.githubusercontent.com/AndreiDrang/python3-anticaptcha/refs/heads/main/files/captcha-image.jpg".to_string(),
            coord_captcha_file: "files/captcha-image-coordinates.jpg".to_string(),
            coord_captcha_url: "https://raw.githubusercontent.com/AndreiDrang/python3-anticaptcha/refs/heads/main/files/captcha-image-coordinates.jpg".to_string(),

            captcha_ends_with: "VDUlyhuxSziimsoTMMVM5MH/2Q==".to_string(),
            coord_captcha_ends_with: "2j4QKPTJGLcIDo9YMfs1AAHSf/2Q==".to_string()
        }
    }
}

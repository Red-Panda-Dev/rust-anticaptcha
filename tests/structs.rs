pub const API_KEY: &str = "SOME_API_KEY";

pub struct ImageCaptchaArgs {
    captcha_file: String,
    captcha_url: String,
    coord_captcha_file: String,
    coord_captcha_url: String,
}

impl ImageCaptchaArgs {
    pub fn new() -> Self {
        ImageCaptchaArgs {
            captcha_file: "files/captcha-image.jpg".to_string(),
            captcha_url : "https://raw.githubusercontent.com/AndreiDrang/python3-anticaptcha/refs/heads/main/files/captcha-image.jpg".to_string(),
            coord_captcha_file: "files/captcha-image-coordinates.jpg".to_string(),
            coord_captcha_url: "https://raw.githubusercontent.com/AndreiDrang/python3-anticaptcha/refs/heads/main/files/captcha-image-coordinates.jpg".to_string()

        }
    }
}

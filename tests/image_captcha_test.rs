use rust_anticaptcha::image_captcha::ImageCaptcha;
use rust_anticaptcha::instruments::image_instrument::ImageInstrument;

use crate::structs::ImageCaptchaArgs;

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
fn image_captcha_instance() {
    let captcha_args = ImageCaptchaArgs::new();

    ImageCaptcha::new(captcha_args.API_KEY);
}

#[test]
fn image_instrument_instance() {
    ImageInstrument::new();
}

#[tokio::test]
async fn image_instrument_read_image_link() {
    let captcha_args = ImageCaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_link(captcha_args.captcha_url)
        .await
        .ends_with(&captcha_args.captcha_ends_with));
}

#[test]
fn image_instrument_read_image_file() {
    let captcha_args = ImageCaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_file(captcha_args.captcha_file)
        .ends_with(&captcha_args.captcha_ends_with));
}

#[tokio::test]
async fn image_instrument_read_coord_image_link() {
    let captcha_args = ImageCaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_link(captcha_args.coord_captcha_url)
        .await
        .ends_with(&captcha_args.coord_captcha_ends_with));
}

#[test]
fn image_instrument_read_coord_image_file() {
    let captcha_args = ImageCaptchaArgs::new();

    let instance = ImageInstrument::new();
    assert!(instance
        .read_image_file(captcha_args.coord_captcha_file)
        .ends_with(&captcha_args.coord_captcha_ends_with));
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! # rust-anticaptcha
//!
//! The `rust-anticaptcha` crate provides Rust async interface for AntiCaptcha service API.
//!
//! It handles all captcha types available at AntiCaptcha service:
//!
//! - Image captcha
//! - ReCaptcha V2 and V3
//! - FunCaptcha
//! - GeeTest
//! - Turnstile
//! - Custom captcha
//! - And additional methods like reporting and get account balance
//!
//! ## Basic example for Image captcha
//! ```no_run
//! use serde_json::json;
//!
//! use rust_anticaptcha::core::enums::ImageTaskType;
//! use rust_anticaptcha::image_captcha::ImageCaptcha;
//!
//! async fn run() {
//!      let map = json!({"body": "base64_string"});
//!      let mut image_to_text_client = ImageCaptcha::new("API_KEY".to_string());
//!      image_to_text_client.captcha_handler(ImageTaskType::ImageToTextTask, &map).await;
//! }
//! ```
//!
//! # Notes
//! Read more here:
//!
//! <https://anti-captcha.com/apidoc>
//!

pub mod core;
pub mod control;
pub mod instruments;
pub mod image_captcha;
pub mod token_captcha;

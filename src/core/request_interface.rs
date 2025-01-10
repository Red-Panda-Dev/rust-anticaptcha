use reqwest::Response;
use serde_json::Value;

use super::constants::BASE_REQUEST_URL;
use super::enums::EnpPostfix;
use super::structs::{CreateTaskRequest, ResultTaskRequest};

pub struct RequestInterface {
    request_client: reqwest::Client,
}
impl Default for RequestInterface {
    fn default() -> Self {
        RequestInterface::new()
    }
}

impl RequestInterface {
    pub fn new() -> Self {
        RequestInterface {
            request_client: reqwest::Client::new(),
        }
    }
    pub async fn send_get_request(&self, url: &String) -> Result<Response, String> {
        let response = self.request_client.get(url).send().await.unwrap();

        if response.status().eq(&200) {
            Ok(response)
        } else {
            Err(format!(
                "Invalid request to {}, status code - {} response - {}",
                url,
                response.status(),
                response.text().await.unwrap()
            ))
        }
    }
    pub async fn send_post_request(
        &self,
        payload: &Value,
        url: &String,
    ) -> Result<Response, String> {
        let response = self.request_client.post(url).json(payload).send().await.unwrap();

        if response.status().eq(&200) {
            Ok(response)
        } else {
            Err(format!(
                "Invalid request  to {}, status code - {} response - {}",
                url,
                response.status(),
                response.text().await.unwrap()
            ))
        }
    }
    pub async fn send_create_task_request(
        &self,
        payload: &CreateTaskRequest,
        enp_postfix: &EnpPostfix,
    ) -> Result<Value, String> {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.to_string();

        let response = self
            .request_client
            .post(&req_url)
            .json(payload)
            .send()
            .await
            .unwrap();

        if response.status().eq(&200) {
            Ok(response.json().await.unwrap())
        } else {
            Err(format!(
                "Invalid request to {}, status code - {} response - {}",
                req_url,
                response.status(),
                response.text().await.unwrap()
            ))
        }
    }
    pub async fn send_get_result_request(
        &self,
        payload: &ResultTaskRequest,
        enp_postfix: &EnpPostfix,
    ) -> Result<Value, String> {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.to_string();

        let response = self
            .request_client
            .post(&req_url)
            .json(payload)
            .send()
            .await
            .unwrap();

        if response.status().eq(&200) {
            Ok(response.json().await.unwrap())
        } else {
            Err(format!(
                "Invalid request to {}, status code - {} response - {}",
                req_url,
                response.status(),
                response.text().await.unwrap()
            ))
        }
    }
}

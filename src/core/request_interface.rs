use std::collections::HashMap;

use super::constants::BASE_REQUEST_URL;
use super::enums::EnpPostfix;
use super::structs::{CreateTaskRequest, ResultTaskRequest};

pub struct RequestInterface {
    task_payload: HashMap<String, String>,
    request_client: reqwest::Client,
}
impl RequestInterface {
    pub fn new() -> Self {
        RequestInterface {
            task_payload: HashMap::new(),
            request_client: reqwest::Client::new(),
        }
    }
    pub async fn send_post_request(
        &self,
        payload: &HashMap<String, String>,
        enp_postfix: &EnpPostfix,
    ) -> serde_json::Value {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.value_as_string();

        let response = self
            .request_client
            .post(req_url)
            .json(payload)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            panic!(
                "Invalid request to API, response - {}",
                response.text().await.unwrap()
            )
        } else {
            response.json().await.unwrap()
        }
    }
    pub async fn send_create_task_request(
        &self,
        payload: &CreateTaskRequest,
        enp_postfix: &EnpPostfix,
    ) -> serde_json::Value {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.value_as_string();

        let response = self
            .request_client
            .post(req_url)
            .json(payload)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            panic!(
                "Invalid request to API, response - {}",
                response.text().await.unwrap()
            )
        } else {
            response.json().await.unwrap()
        }
    }
    pub async fn send_get_result_request(
        &self,
        payload: &ResultTaskRequest,
        enp_postfix: &EnpPostfix,
    ) -> serde_json::Value {
        let req_url = BASE_REQUEST_URL.to_string() + &enp_postfix.value_as_string();

        let response = self
            .request_client
            .post(req_url)
            .json(payload)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            panic!(
                "Invalid request to API, response - {}",
                response.text().await.unwrap()
            )
        } else {
            response.json().await.unwrap()
        }
    }
}

use crate::utils::InitVideoResponse;
use ::reqwest::Body;
use ::reqwest::Client;
use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::borrow::Borrow;

pub async fn video_request(
    url: String,
    body: Value,
    token: String,
) -> Result<InitVideoResponse, String> {
    let req = Client::new().post(url);

    let bear_token = "Bearer ".to_owned() + &token;
    let resp = if token.is_empty() {
        req.header("X-Restli-Protocol-Version", "2.0.0")
            .json(&body)
            .send()
            .await
    } else {
        // when is initializing of upload.
        let bear_token = "Bearer ".to_owned() + &token;
        req.header("X-Restli-Protocol-Version", "2.0.0")
            .header("Content-Type", "application/json")
            .header("Authorization", bear_token)
            .json(&body)
            .send()
            .await
    };

    match resp {
        Ok(response) => {
            let content = response.json::<InitVideoResponse>().await;

            match content {
                Ok(res) => Ok(res),
                Err(er) => {
                    println!("result: {:?}", er);

                    Err("error".to_string())
                }
            }
        }
        Err(er) => {
            println!("error ddddd: {:?}", er);
            Err("error".to_string())
        }
    }
}
pub async fn video_request_final(
    url: String,
    body: Value,
    token: String,
) -> Result<String, String> {
    println!("url : {}", url);

    let req = Client::new().post(url);

    let bear_token = "Bearer ".to_owned() + &token;
    let resp = if token.is_empty() {
        req.header("X-Restli-Protocol-Version", "2.0.0")
            .json(&body)
            .send()
            .await
    } else {
        // when is initializing of upload.
        let bear_token = "Bearer ".to_owned() + &token;
        req.header("X-Restli-Protocol-Version", "2.0.0")
            .header("Content-Type", "application/json")
            .header("Authorization", bear_token)
            .json(&body)
            .send()
            .await
    };

    match resp {
        Ok(response) => {
            let content = response.text().await;
            Ok("".to_string())
        }
        Err(er) => {
            println!("error : {:?}", er);
            Err("error".to_string())
        }
    }
}

pub async fn upload_chunk_as_bytes(
    url: String,
    body: Vec<u8>,
    token: String,
    method: &str,
) -> Result<String, String> {
    let bear_token = "Bearer ".to_owned() + &token;
    let new_body = body.to_vec();

    let resp = if method == "PUT" {
        //upload by chunking
        Client::new()
            .patch(url)
            .header("Content-Type", "application/octet-stream")
            .header("X-Restli-Protocol-Version", "2.0.0")
            .header("Authorization", bear_token)
            .body(Body::from(new_body))
            .send()
            .await
    } else {
        Client::new()
            .post(url)
            .header("Authorization", bear_token)
            .header("X-Restli-Protocol-Version", "2.0.0")
            .body(Body::from(new_body))
            .send()
            .await
    };

    match resp {
        Ok(response) => {
            if let Some(etag_header) = response.headers().get("etag") {
                let etag = etag_header
                    .to_str()
                    .expect("cant open etag header")
                    .to_owned();
                Ok(etag)
            } else {
                Err("error".to_string())
            }
        }
        Err(err) => Err("error".to_string()),
    }
}

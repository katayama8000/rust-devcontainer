use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<ResponseItem>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseItem {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub details: Value,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Error {
    InvalidArgument(String),
    ExpoErr(ErrorResponse),
    DeserializeErr(String),
    Others(String),
}

#[derive(Debug, Serialize)]
pub struct PushPayload<'a> {
    to: &'a [&'a str],
    title: &'a str,
    body: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushTicket {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct PushResult {
    data: Vec<ResultEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Details {
    error: String,
}

#[derive(Debug, Deserialize)]
pub struct ResultEntry {
    status: String,
    id: Option<String>,
    message: Option<String>,
    details: Option<Details>,
}

pub async fn push_message(
    expo_push_tokens: &[&str],
    title: &str,
    body: &str,
) -> Result<PushResult, Error> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for token in expo_push_tokens {
        if !token.starts_with("ExponentPushToken[") {
            return Err(Error::InvalidArgument(format!(
                "Invalid expo push token: {}",
                token
            )));
        }
    }

    if title.is_empty() {
        return Err(Error::InvalidArgument("Title is empty".to_string()));
    }

    if body.is_empty() {
        return Err(Error::InvalidArgument("Body is empty".to_string()));
    }

    let mut result_entries = Vec::new();

    match client
        .post(URL)
        .headers(headers)
        .json(&PushPayload {
            to: expo_push_tokens,
            title,
            body,
        })
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                for data in response.json::<PushResult>().await.unwrap().data {
                    if data.status == "ok" {
                        result_entries.push(ResultEntry {
                            status: "ok".to_string(),
                            id: Some(data.id.unwrap()),
                            message: None,
                            details: None,
                        });
                    } else {
                        result_entries.push(ResultEntry {
                            status: "error".to_string(),
                            id: None,
                            message: Some(data.message.unwrap()),
                            details: Some(Details {
                                error: data.details.unwrap().error,
                            }),
                        });
                    }
                }
                Ok(PushResult {
                    data: result_entries,
                })
            } else {
                let response = response.json::<ErrorResponse>().await.map_err(|err| {
                    Error::DeserializeErr(format!(
                        "Failed to parse response body as ErrorResponse: {:?}",
                        err
                    ))
                })?;
                Err(Error::ExpoErr(response))
            }
        }
        Err(err) => Err(Error::Others(format!("Failed to send request: {:?}", err))),
    }
}

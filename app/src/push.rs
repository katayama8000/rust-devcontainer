use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<ResponseItem>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseItem {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SuperResponse {
    PushTicket(PushTicket),
    ErrorResponse(ErrorResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub details: Details,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Details {
    pub error: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Error {
    InvalidArgument(String),
    ExpoErr(String),
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
struct PushResult {
    data: Vec<ResultEntry>,
}

#[derive(Debug, Deserialize)]
struct ResultEntry {
    status: String,
    id: Option<String>,
    message: Option<String>,
    details: Option<Details>,
}

pub async fn push_message(
    expo_push_tokens: &[&str],
    title: &str,
    body: &str,
) -> Result<Vec<SuperResponse>, Error> {
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

    let payload = PushPayload {
        to: expo_push_tokens,
        title,
        body,
    };

    match client
        .post(URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let result: Vec<SuperResponse> = response
                    .json::<PushResult>()
                    .await
                    .map_err(|err| {
                        Error::DeserializeErr(format!(
                            "Failed to parse response body as PushResult: {:?}",
                            err
                        ))
                    })?
                    .data
                    .into_iter()
                    .map(|item| {
                        if item.status == "error" {
                            SuperResponse::ErrorResponse(ErrorResponse {
                                status: item.status,
                                message: item.message.unwrap_or_default(), // Use unwrap_or_default to provide a default value
                                details: item.details.unwrap_or_default(), // Use unwrap_or_default to provide a default value
                            })
                        } else if item.status == "ok" {
                            SuperResponse::PushTicket(PushTicket {
                                status: "ok".to_string(),
                                id: item.id.unwrap_or_default(), // Use unwrap_or_default to provide a default value
                            })
                        } else {
                            unreachable!("Unknown status: {}", item.status)
                        }
                    })
                    .collect();

                Ok(result)
            } else {
                Err(Error::ExpoErr(format!(
                    "Failed to send request: {:?}",
                    response
                )))
            }
        }
        Err(err) => Err(Error::Others(format!("Failed to send request: {:?}", err))),
    }
}

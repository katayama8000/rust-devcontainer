use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, PartialEq)]
pub enum CustomError {
    InvalidArgument(String),
    DeserializeErr(String),
    ServerErr(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PushTicket {
    Success(PushSuccessTicket),
    Error(PushErrorTicket),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushSuccessTicket {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushErrorTicket {
    pub status: String,
    pub message: String,
    pub details: Value,
}

#[derive(Debug, Serialize)]
pub struct PushPayload {
    to: Vec<String>,
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct PushResult {
    data: Vec<PushResultItem>,
}

#[derive(Debug, Deserialize)]
struct PushResultItem {
    status: String,
    id: Option<String>,
    message: Option<String>,
    details: Option<Value>,
}

pub async fn send_push_notification(
    expo_push_tokens: Vec<String>,
    title: String,
    body: String,
) -> Result<Vec<PushTicket>, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for token in expo_push_tokens.clone() {
        if !token.starts_with("ExponentPushToken[") {
            return Err(CustomError::InvalidArgument(format!(
                "Invalid expo push token: {}",
                token
            )));
        }
    }

    if title.is_empty() {
        return Err(CustomError::InvalidArgument("Title is empty".to_string()));
    }

    if body.is_empty() {
        return Err(CustomError::InvalidArgument("Body is empty".to_string()));
    }

    let payload = json!({
        "to": expo_push_tokens,
        "title": title,
        "body": body,
    });

    match client
        .post(URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response
                    .json::<PushResult>()
                    .await
                    .map_err(|err| {
                        CustomError::DeserializeErr(format!(
                            "Failed to deserialize response: {:?}",
                            err
                        ))
                    })?
                    .data
                    .into_iter()
                    .map(|item| {
                        if item.status == "error" {
                            PushTicket::Error(PushErrorTicket {
                                status: item.status,
                                message: item.message.expect("message is empty"),
                                details: item.details.expect("details is empty"),
                            })
                        } else if item.status == "ok" {
                            PushTicket::Success(PushSuccessTicket {
                                status: item.status,
                                id: item.id.expect("id is empty"),
                            })
                        } else {
                            unreachable!("Unknown status: {}", item.status)
                        }
                    })
                    .collect())
            } else {
                Err(CustomError::ServerErr(format!(
                    "Failed to send request: {:?}",
                    response
                )))
            }
        }
        Err(err) => Err(CustomError::ServerErr(format!(
            "Failed to send request: {:?}",
            err
        ))),
    }
}

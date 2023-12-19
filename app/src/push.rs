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

#[derive(Debug, Serialize, Deserialize)]
pub enum SuperResponse {
    Ticket(PushTicket),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PushTicket {
    pub status: String,
    pub id: String,
}

impl std::fmt::Display for PushTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ status: {}, id: {} }}", self.status, self.id)
    }
}

pub async fn push_message(expo_push_tokens: &[&str], title: &str, body: &str) -> Result<(), Error> {
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
                print!("🔥 1 🔥");
                let response = response.json::<ApiResponse>().await.map_err(|err| {
                    Error::DeserializeErr(format!(
                        "Failed to parse response body as ApiResponse: {:?}",
                        err
                    ))
                });

                print!("🔥 2 🔥");
                println!("{:?}", response);
                // let result: Vec<SuperResponse> = response
                //     .unwrap()
                //     .into_iter()
                //     .map(|item| {
                //         if item.status == "error" {
                //             print!("🔥 3 🔥");
                //             println!("{:?}", item);
                //             SuperResponse::Error(ErrorResponse {
                //                 status: item.status,
                //                 message: String::new(), // Add a default value for message
                //                 details: Value::Null,   // Add a default value for details
                //             })
                //         } else {
                //             print!("🧊 3 🧊");
                //             println!("{:?}", item);
                //             SuperResponse::Ticket(PushTicket {
                //                 status: "ok".to_string(),
                //                 id: item.id.clone(),
                //             })
                //         }
                //     })
                //     .collect();

                Ok(())

                // let response = response.json::<Response>().await.map_err(|err| {
                //     Error::DeserializeErr(format!(
                //         "Failed to parse response body as ApiResponse: {:?}",
                //         err
                //     ))
                // })?;

                // let result: Vec<Response> = response
                //     .data
                //     .into_iter()
                //     .map(|item| {
                //         if item.status == "error" {
                //             print!("🔥");
                //             Response::Error(ErrorResponse {
                //                 status: item.status,
                //                 message: String::new(), // Add a default value for message
                //                 details: Value::Null,   // Add a default value for details
                //             })
                //         } else {
                //             print!("🧊");
                //             Response::Ticket(PushTicket {
                //                 status: "ok".to_string(),
                //                 id: item.id.clone(),
                //             })
                //         }
                //     })
                //     .collect();

                // Ok(result)
            } else {
                print!("🧊");
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

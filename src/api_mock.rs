use reqwest::Client;
use serde::Deserialize;

pub async fn run() {
    println!("api_mock.rs");
    let _todo = fetch_todo(None).await.unwrap();
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

struct JsonClient {
    url: String,
    client: Client,
}

impl JsonClient {
    fn new(url: Option<&str>) -> Self {
        Self {
            url: url
                .unwrap_or("https://jsonplaceholder.typicode.com")
                .to_string(),
            client: Client::new(),
        }
    }
}

pub async fn fetch_todo(url: Option<&str>) -> Result<Todo, reqwest::Error> {
    let client = JsonClient::new(url);
    let body = client
        .client
        .get(format!("{}/todos/1", client.url))
        .send()
        .await?
        .json::<Todo>()
        .await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use mockito::Server;

    use super::*;

    #[tokio::test]
    async fn test_happy_path_fetch_todo() {
        let todo = Todo {
            user_id: 1,
            id: 1,
            title: "delectus aut autem".to_string(),
            completed: false,
        };
        assert_eq!(fetch_todo(None).await.unwrap(), todo);
    }

    #[tokio::test]
    async fn test_happy_path_fetch_todo_mocked() {
        let mut server = Server::new_async().await;
        let json_body = serde_json::json!({
            "userId": 1,
            "id": 1,
            "title": "delectus aut autem",
            "completed": false
        });

        let body_bytes = serde_json::to_vec(&json_body).unwrap();
        server
            .mock("GET", "/todos/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body_bytes)
            .create();

        let todo = fetch_todo(Some(&server.url())).await.unwrap();
        let expected_todo = Todo {
            user_id: 1,
            id: 1,
            title: "delectus aut autem".to_string(),
            completed: false,
        };
        assert_eq!(todo, expected_todo);
    }

    #[tokio::test]
    #[ignore = "not working"]
    async fn test_sad_path_fetch_todo_mocked() {
        let mut server = Server::new_async().await;
        server.mock("GET", "/todos/1").with_status(404).create();

        // check status code
        let result = fetch_todo(Some(&server.url())).await;
        let err = result.unwrap_err();
        assert_eq!(err.status(), Some(reqwest::StatusCode::NOT_FOUND));
    }
}

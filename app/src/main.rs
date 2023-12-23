use expo_server_sdk::{Expo, ExpoPushMessage, ExpoPushTicket};

#[tokio::main]
async fn main() {
    let expo_push_tokens = vec![
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]".to_string(),
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]".to_string(),
    ];
    let title = "Hello".to_string();
    let body = "World".to_string();

    // let mut data = HashMap::new();
    // data.insert("key1".to_string());

    let expo_message = ExpoPushMessage::new(expo_push_tokens, title, body)
        .badge(10)
        .mutable_content(true);

    let expo = Expo::new(expo_server_sdk::expo::expo::ExpoClientOptions { access_token: None });
    let result: Result<Vec<ExpoPushTicket>, expo_server_sdk::error::CustomError> =
        expo.send_push_notifications(expo_message).await;
    println!("{:?}", result)
}

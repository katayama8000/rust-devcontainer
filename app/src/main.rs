use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage};

#[tokio::main]
async fn main() {
    let expo_push_tokens = vec![
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]".to_string(),
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]".to_string(),
    ];
    let title = "Hello".to_string();
    let body = "World".to_string();

    let expo_message = ExpoPushMessage::new(expo_push_tokens, title, body);

    let expo = Expo::new(ExpoClientOptions { access_token: None });
    let result = expo.send_push_notifications(expo_message).await;
    println!("{:?}", result);
}

use expo_server_sdk::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushReceiptId};

#[tokio::main]
async fn main() {
    let access_token = "xxxxx".to_string();
    let expo_push_tokens = vec![
        "ExponentPushToken[xxxxx]".to_string(),
        "ExponentPushToken[xxxxx]".to_string(),
    ];
    let title = "Hello".to_string();
    let body = "World".to_string();

    let expo = Expo::new(ExpoClientOptions {
        access_token: Some(access_token),
    });
    let expo_push_message = ExpoPushMessage::new(expo_push_tokens, title, body);
    expo.send_push_notifications(expo_push_message).await;

    let expo_push_ids = ExpoPushReceiptId::new(vec!["xxxxx".to_string(), "xxxxx".to_string()]);
    expo.get_push_notification_receipts(expo_push_ids).await;
}

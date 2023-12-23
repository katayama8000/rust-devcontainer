use std::collections::HashMap;

use expo_server_sdk::{expo::expo::Expo, object::expo_push_message::ExpoPushMessage};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;
use serde_with::skip_serializing_none;

#[tokio::main]
async fn main() {
    let expo_push_tokens = vec![
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]".to_string(),
        // "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]".to_string(),
    ];
    let title = "Hello".to_string();
    let body = "World".to_string();

    // let expo_message = ExpoPushMessage::new(expo_push_tokens, title, body);

    // let expo = Expo::new(expo_server_sdk::expo::expo::ExpoClientOptions { access_token: None });
    // let result = expo.send_push_notifications(expo_message).await;
    // println!("{:?}", result)

    let message = ExpoPushMessage {
        to: vec!["user1".to_string(), "user2".to_string()],
        title: "My title".to_string(),
        body: "My body".to_string(),
        data: Some(HashMap::new()),
        ttl: None,
        expiration: None,
        priority: None,
        subtitle: None,
        sound: None,
        badge: None,
        channel_id: None,
        category_id: None,
        mutable_content: None,
    };

    #[skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
    struct Hoge {
        val1: i32,
        val2: Option<u64>,
    }
    let hoge = Hoge {
        val1: 1,
        val2: None,
    };
    // jsonに変換
    let json = serde_json::to_string(&hoge).unwrap();
    println!("{}", json);
}

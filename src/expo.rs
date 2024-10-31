use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage};
use serde::Serialize;
// async fn send_push_notifications() -> Result<(), Box<dyn std::error::Error>> {
//     // Initialize the Expo client
//     let expo = Expo::new(ExpoClientOptions { access_token: None });
//     let push_tokens = vec!["ExponentPushToken[8lKlc5NYEhh2oCQpYhEZVn]"];

//     #[derive(Debug, Serialize)]
//     struct Data {
//         data: String,
//     }

//     println!("PUSH NOTIFICATIONS TO {:?}", push_tokens);
//     // Build Expo Push Message with detailed configurations
//     let expo_push_message = ExpoPushMessage::builder(push_tokens)
//         .body("body")
//         .data(&Data {
//             data: "data".to_string(),
//         })?
//         .ttl(100)
//         .priority("high")
//         .subtitle("subtitle")
//         .sound("default")
//         .badge(1)
//         .channel_id("channel_id")
//         .category_id("category_id")
//         .mutable_content(true)
//         .title("title")
//         .build()?;
//     let tickets = expo.send_push_notifications(expo_push_message).await;

//     match tickets {
//         Ok(receipts) => println!("PUSH NOTIFICATIONS TICKETS {:?}", receipts),
//         Err(e) => eprintln!("Failed to send push notifications: {:?}", e),
//     }
//     Ok(())
// }

pub async fn run() {
    println!("expo.rs");
    send_push_notifications(vec!["ExponentPushToken[8lKlc5NYEhh2oCQpYhEZVn]".to_string()])
        .await
        .unwrap();
}

async fn send_push_notifications(
    push_tokens: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Expo client
    let expo = Expo::new(ExpoClientOptions { access_token: None });

    println!("PUSH NOTIFICATIONS TO {:?}", push_tokens);

    #[derive(Serialize)]
    struct Data {
        data: String,
    }

    // Build Expo Push Message with detailed configurations
    let expo_push_message = ExpoPushMessage::builder(push_tokens)
        .body("body")
        .data(&Data {
            data: "data".to_string(),
        })?
        .ttl(100)
        .priority("high")
        .subtitle("subtitle")
        .sound("default")
        .badge(1)
        .channel_id("channel_id")
        .category_id("category_id")
        .mutable_content(true)
        .title("title")
        .build()?;
    let tickets = expo.send_push_notifications(expo_push_message).await;

    match tickets {
        Ok(receipts) => println!("PUSH NOTIFICATIONS TICKETS {:?}", receipts),
        Err(e) => eprintln!("Failed to send push notifications: {:?}", e),
    }
    Ok(())
}

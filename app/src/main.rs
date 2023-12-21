use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum CustomError {
    InvalidArgument(String),
    DeserializeErr(String),
    ServerErr(String),
}

pub mod get;
pub mod push;
use get::get_push_receipts;
use push::{send_push_notification, PushTicket};

use crate::get::PushReceipt;

// Assuming `CustomError` and other necessary modules are imported appropriately.

#[tokio::main]
async fn main() {
    // Replace these values with actual Expo Push Tokens and other relevant data

    let expo_push_tokens = [
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]",
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0s]",
    ];
    let title = "Hello";
    let body = "World";

    // Example of sending a push notification
    let ids: Vec<String> = match send_push_notification(
        expo_push_tokens.iter().map(|s| s.to_string()).collect(),
        title.to_string(),
        body.to_string(),
    )
    .await
    {
        Ok(push_tickets) => {
            println!("Push notification sent successfully!");
            let mut receipt_ids = Vec::new();
            for push_ticket in push_tickets {
                match push_ticket {
                    PushTicket::Success(success_ticket) => {
                        println!("Success Ticket: {:?}", success_ticket);
                        receipt_ids.push(success_ticket.id);
                    }
                    PushTicket::Error(error_ticket) => {
                        println!("Error Ticket: {:?}", error_ticket);
                    }
                }
            }
            receipt_ids
        }
        Err(error) => {
            eprintln!("Failed to send push notification: {:?}", error);
            vec![]
        }
    };

    // Example of getting push receipts
    match get_push_receipts(ids.clone()).await {
        Ok(push_receipts) => {
            println!("Push receipts received successfully!");
            for push_receipt in push_receipts {
                match push_receipt {
                    PushReceipt::Success(success_receipt) => {
                        println!("Success Receipt: {:?}", success_receipt);
                    }
                    PushReceipt::Error(error_receipts) => {
                        println!("Error Receipts: {:?}", error_receipts);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Failed to get push receipts: {:?}", error);
        }
    }
}

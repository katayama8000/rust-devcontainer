use lib_demo::{add, print_random_number};
use push::{push_message, Error, PushTicket};
mod push;

#[tokio::main]
async fn main() {
    add(1, 1);
    print_random_number();
    let push_token = [
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]",
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0s]",
    ];
    let title = "Hello";
    let body = "World";

    let ret: Result<Vec<String>, Error> = match push_message(&push_token, title, body).await {
        Ok(super_responses) => {
            let mut ids = Vec::new();
            for super_response in super_responses {
                match super_response {
                    PushTicket::Success(push_ticket) => {
                        println!("PushTicket: {:?}", push_ticket);
                        ids.push(push_ticket.id);
                    }
                    PushTicket::Error(error_response) => {
                        println!("Error: {:?}", error_response);
                    }
                }
            }
            Ok(ids)
        }
        Err(error) => Err(error),
    };
    println!("ret: {:?}", ret.unwrap());
}

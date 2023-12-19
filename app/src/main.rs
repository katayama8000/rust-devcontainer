use lib_demo::{add, print_random_number};
mod push;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
    print_random_number();
    let title = "sdk-test-title";
    let body = "sdk-test-body";
    let push_token = [
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0sB]",
        "ExponentPushToken[GG5W7qB0nelNDkz5Y6A0s]",
    ];
    let ret = match push::push_message(&push_token, title, body).await {
        Ok(val) => val,
        Err(_e) => todo!(),
    };
    println!("main.rs => {:?}", ret);
}

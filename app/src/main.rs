use expo_server_sdk::{get_receipts, push_message};
use lib_demo::{add, print_random_number};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
    print_random_number();
    let title = "sdk-test-title";
    let body = "sdk-test-body";
    let push_token = "ExponentPushToken[b5nR6zALafV431QtOC7b";
    let ret = match push_message(&[push_token], title, body).await {
        Ok(val) => val,
        Err(_e) => todo!(),
    };
    println!("{}", ret.data[0].id)
}

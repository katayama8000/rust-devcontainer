use tokio::time::{sleep, Duration};

// そのまま実行すると、以下のエラーが出る
// Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.
// main から #[tokio::main] と async をコメントアウトして、実行してください

#[tokio::main]
pub async fn run() {
    println!("Starting the program...");

    // async task 1
    let handle1 = tokio::spawn(async {
        println!("Task 1: Started");
        sleep(Duration::from_secs(2)).await;
        println!("Task 1: Completed");
        42
    });

    // async task 2
    let handle2 = tokio::spawn(async {
        println!("Task 2: Started");
        sleep(Duration::from_secs(1)).await;
        println!("Task 2: Completed");
        84
    });

    let result1 = handle1.await.expect("Task 1 panicked");
    let result2 = handle2.await.expect("Task 2 panicked");

    println!("Task 1 returned: {}", result1);
    println!("Task 2 returned: {}", result2);

    println!("Program completed.");
}

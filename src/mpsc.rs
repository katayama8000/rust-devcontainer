use tokio::sync::mpsc;

pub async fn run() {
    // 1. 容量100のmpscチャネルを作成
    let (tx, mut rx) = mpsc::channel::<i32>(100);

    // 2. 送信機をクローンして、複数のタスクで使用
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // 3. 送信タスク1
    tokio::spawn(async move {
        tx1.send(10).await.expect("Failed to send 10");
        println!("Task 1 sent 10");
    });

    // 4. 送信タスク2
    tokio::spawn(async move {
        tx2.send(20).await.expect("Failed to send 20");
        println!("Task 2 sent 20");
    });
    
    // 5. 受信タスク (メインタスク)
    // tx がドロップされるまで、チャネルからメッセージを受信し続ける
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
    
    // 最後に残った元の tx をドロップ
    drop(tx); 

    println!("Channel closed. Main task finished.");
}

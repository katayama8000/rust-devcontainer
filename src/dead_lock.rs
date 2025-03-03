use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run() {
    // 2つのMutexをラップしたArcを作成
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    // mutex1とmutex2の所有権をcloneする
    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);

    // スレッド1: mutex1をロックしてからmutex2をロックしようとする
    let thread1 = thread::spawn(move || {
        let _guard1 = mutex1.lock().unwrap();
        println!("Thread 1: mutex1 locked, waiting for mutex2...");
        thread::sleep(Duration::from_millis(100));
        let _guard2 = mutex2_clone.lock().unwrap();
        println!("Thread 1: mutex2 locked.");
    });

    // スレッド2: mutex2をロックしてからmutex1をロックしようとする
    let thread2 = thread::spawn(move || {
        let _guard2 = mutex2.lock().unwrap();
        println!("Thread 2: mutex2 locked, waiting for mutex1...");
        thread::sleep(Duration::from_millis(100));
        let _guard1 = mutex1_clone.lock().unwrap();
        println!("Thread 2: mutex1 locked.");
    });

    // スレッドの終了を待つ
    thread1.join().unwrap();
    thread2.join().unwrap();

    // ここには到達しない
    println!("Deadlock avoided!");
}

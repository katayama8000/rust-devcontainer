use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    // // 共有される可変なデータを保持するMutexを作成します。
    // let counter = Arc::new(Mutex::new(0));

    // let mut handles = vec![];

    // for _ in 0..10 {
    //     // スレッドごとにMutexのクローンを作成し、Arcで共有します。
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         // Mutexのロックを取得し、共有データにアクセスします。
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // // すべてのスレッドの実行が終了するまで待機します。
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    let mutex = Mutex::new(0);
    println!("{:?}", mutex);
    let mut data = mutex.lock().unwrap();
    println!("{:?}", data);
    // update value
    *data = 1;
    println!("{:?}", data);
}

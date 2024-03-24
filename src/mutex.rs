use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn run() {
    // run1();
    // run2();
    run3();
}

fn run1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("counter: {:?}", counter);
            println!("num: {:?}", num);

            // wait for 1 second
            thread::sleep(std::time::Duration::from_secs(1));
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn run2() {
    let counter = Arc::new(Mutex::new(0));

    let counter1 = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        println!("thread1 started");
        let mut num = counter1.lock().unwrap();
        println!("counter: {:?}", counter1);
        println!("num: {:?}", num);
        *num += 1;
        drop(num);

        // wait for 3 seconds
        thread::sleep(std::time::Duration::from_secs(3));
    });

    let counter2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        println!("thread2 started");
        let mut num = counter2.lock().unwrap();
        println!("counter: {:?}", counter2);
        println!("num: {:?}", num);
        *num += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result: {}", *counter.lock().unwrap());
}

fn run3() {
    // sample of deadlock
    let counter = Mutex::new(0);
    let lock1 = counter.lock().unwrap();
    let lock2 = counter.lock().unwrap();
    println!("lock: {:?}", lock1);
}

use std::sync::Arc;
use std::thread;

pub fn run() {
    let rc = Arc::new(42);
    let thread = thread::spawn(move || {
        eprintln!("value = {}", rc);
    });
    thread.join().unwrap();
}

pub fn run() {
    println!("{}{}{}thread.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    // make a new thread
    let handle = std::thread::spawn(|| {
        println!("I am a new thread!");
    });

    // wait for the thread to finish. Returns a result.
    handle.join().unwrap();
}

use std::sync::RwLock;
pub fn run() {
    println!(
        "{}{}{}raw_lock.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("r1: {}, r2: {}", r1, r2);
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("w: {}", w);
    }
    println!("lock: {:?}", lock);
}

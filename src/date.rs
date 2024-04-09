pub fn run() {
    // println!("date.rs");
    // let now = chrono::Utc::now();
    // println!("now: {}", now);
    // let nano_now = now.trunc_subsecs(6);
    // println!("now: {}", nano_now);
    // let micro_now = now.trunc_subsecs(3);
    // println!("now: {}", micro_now);
    let now = chrono::Utc::now();
    println!("now: {}", now);
    let milli = now.timestamp_millis();
    println!("milli: {}", milli);
}

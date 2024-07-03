pub fn run() {
    println!("{}{}{}str.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let s = "hello"; // 5bytes
    let t = "world"; // 5bytes
    println!("s: {:p}", &s);
    println!("t: {:p}", &t);
    // 16bytes (8bytes pointer, 8bytes length)
    // check pointer
    println!("s: {:p}", s.as_ptr());
    // check length
    println!("s: {}", s.len());
    // String
    let mut s = String::from("hello");
    let t = String::from("world");
    println!("s: {:p}", &s);
    println!("t: {:p}", &t);
    // 24bytes (8bytes pointer, 8bytes length, 8bytes capacity)
    println!("s: {:p}", s.as_ptr());
    println!("s: {}", s.len());
    println!("s: {}", s.capacity());
    s.push_str(" world");
    println!("s: {}", s.len());
    println!("s: {}", s.capacity());
    // ---
    let s = "hello";
    // &str -> String
    let t = s.to_string();
    let t = s.to_owned();
}

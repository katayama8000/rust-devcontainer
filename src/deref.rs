pub fn run() {
    println!("deref.rs");
    let option = Some("Hello".to_string());
    let result = option.as_deref();
    // asref.rs
    let result = option.as_ref();
    let str = "Hello".to_string();
    let result: &[u8] = str.as_ref();
    println!("{:?}", result);
}

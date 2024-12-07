pub fn run() {
    // println!("{}{}{}str.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    // let s = "hello"; // 5bytes
    // let t = "world"; // 5bytes
    // println!("s: {:p}", &s);
    // println!("t: {:p}", &t);
    // // 16bytes (8bytes pointer, 8bytes length)
    // // check pointer
    // println!("s: {:p}", s.as_ptr());
    // // check length
    // println!("s: {}", s.len());
    // // String
    // let mut s = String::from("hello");
    // let t = String::from("world");
    // println!("s: {:p}", &s);
    // println!("t: {:p}", &t);
    // // 24bytes (8bytes pointer, 8bytes length, 8bytes capacity)
    // println!("s: {:p}", s.as_ptr());
    // println!("s: {}", s.len());
    // println!("s: {}", s.capacity());
    // s.push_str(" world");
    // println!("s: {}", s.len());
    // println!("s: {}", s.capacity());
    // // ---
    // let s = "hello";
    // // &str -> String
    // let t = s.to_string();
    // let t = s.to_owned();

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // error! （エラー！）

    // println!("the first word is: {}", word);

    let s1 = String::from("hello world");
    fun(s1);

    let s2: &str = "hello world";
    let s3 = Into::<String>::into(s2);
    fun(s2);
}

fn fun(s: impl Into<String>) {
    let s = s.into();
    println!("{}", s);
}

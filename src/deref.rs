use std::{ops::Deref, result};

use serde::de;

pub fn run() {
    println!("deref.rs");
    let option = Some("Hello".to_string());
    let result = option.as_deref();
    // asref.rs
    let result = option.as_ref();
    let str = "Hello".to_string();
    let result: &[u8] = str.as_ref();
    println!("{:?}", result);

    let num = Some(10);
    let result = num.as_ref();
    println!("{:?}", result);
    let deref = result.as_deref();

    let my_visitor = MyVisitor { value: 10 };
    let result = *my_visitor;
    println!("{:?}", result);

    let my_visitor2 = MyVisitor2 { value: 20 };
    let result: &i32 = my_visitor2.deref();
}

struct MyVisitor {
    value: i32,
}

impl Deref for MyVisitor {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct MyVisitor2 {
    value: i32,
}

impl Deref for MyVisitor2 {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

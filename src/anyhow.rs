use std::fs::File;

use anyhow::Context;

pub fn run() {
    println!("anyhow.rs");
    let val = convert_str_to_i32("a").unwrap();
    // println!("val: {}", val);
    parant_function().unwrap();
}

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

fn convert_str_to_i32(val: &str) -> Result<i32, MyError> {
    val.parse::<i32>().map_err(MyError::Num)
}

fn convert_str_to_i32_v2(val: &str) -> Result<i32, anyhow::Error> {
    val.parse::<i32>().map_err(|e| anyhow::Error::new(e))
    // val.parse::<i32>()
    //     .map_err(|e| anyhow::Error::msg("Failed to parse i32"))
    // File::open("foo.txt").context("error")
}

fn parant_function() -> Result<i32, anyhow::Error> {
    let val = convert_str_to_i32_v2("j")?;
    Ok(val)
}

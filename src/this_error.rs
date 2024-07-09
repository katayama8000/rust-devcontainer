use std::result;

use thiserror::Error;

pub fn run() {
    println!("this_error.rs");
    match main_function() {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Err: {:?}", e),
    }
}

#[derive(Debug, Error)]
enum MyError1 {
    #[error("Error1 occurred")]
    Error1,
    #[error("Error2 occurred")]
    Error2,
}

#[derive(Debug, Error)]
enum MyError2 {
    #[error("Error3 occurred")]
    Error3,
    #[error("Error4 occurred")]
    Error4,
}

#[derive(Debug, Error)]
enum MainError {
    #[error("MainError occurred")]
    MainError(#[from] MyError1),
    #[error("SubError occurred")]
    SubError(#[from] MyError2),
    #[error("ParseError occurred")]
    ParseError(#[from] ParseError),
}

// error の変換
// impl From<MyError1> for MyError2 {
//     fn from(error: MyError1) -> Self {
//         match error {
//             MyError1::Error1 => MyError2::Error3,
//             MyError1::Error2 => MyError2::Error4,
//         }
//     }
// }

fn sample1() -> Result<(), MyError1> {
    Err(MyError1::Error1)
}

fn sample2() -> Result<(), MyError2> {
    Ok(())
}

fn main_function() -> Result<(), MainError> {
    let result1 = sample1()?;
    let result2 = sample2()?;
    Ok(result2)
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("ParsedIntError occurred")]
    ParsedIntError,
}

fn parse() -> Result<(), ParseError> {
    Err(ParseError::ParsedIntError)
}

fn map_error() -> Result<(), MainError> {
    let result = parse()?;
    Ok(result)
}

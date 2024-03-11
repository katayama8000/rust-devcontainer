pub fn run() {
    println!("custom_error.rs");
    match my_function2() {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Err: {:?}", e),
    }
}

enum CustomError1 {
    ParsedIntError,
}

#[derive(Debug)]
enum CustomError2 {
    UnkownError,
}

impl From<CustomError1> for CustomError2 {
    fn from(error: CustomError1) -> Self {
        CustomError2::UnkownError
    }
}

fn my_function1() -> Result<(), CustomError1> {
    Err(CustomError1::ParsedIntError)
}

fn my_function2() -> Result<(), CustomError2> {
    let result = my_function1()?;
    Ok(result)
}

fn sample1() -> Result<i32, i32> {
    let result = sample2()?;
    Ok(result)
}

fn sample2() -> Result<i32, i32> {
    Ok(1)
}

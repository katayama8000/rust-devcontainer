use num_traits::NumOps;
use std::str::FromStr;

pub fn run() {
    println!("custom_error2.rs");
    let val = parse_value::<i32>("1.0").unwrap();
    let err = parse_value::<f32>("1").unwrap();
    println!("val: {}", val);
    println!("err: {}", err);
}

#[derive(Debug)]
pub enum SampleError {
    IntError(std::num::ParseIntError),
    FloatError(std::num::ParseFloatError),
}

impl std::error::Error for SampleError {}

impl std::fmt::Display for SampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SampleError::IntError(e) => write!(f, "IntError: {}", e),
            SampleError::FloatError(e) => write!(f, "FloatError: {}", e),
        }
    }
}

impl From<std::num::ParseIntError> for SampleError {
    fn from(error: std::num::ParseIntError) -> Self {
        SampleError::IntError(error)
    }
}

impl From<std::num::ParseFloatError> for SampleError {
    fn from(error: std::num::ParseFloatError) -> Self {
        SampleError::FloatError(error)
    }
}

fn parse_value<T>(value: &str) -> Result<T, SampleError>
where
    T: NumOps + FromStr,
    SampleError: From<<T as FromStr>::Err>,
{
    let result = value.parse::<T>().map_err(SampleError::from)?;
    Ok(result)
}

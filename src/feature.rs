use lib_demo::second;

pub fn run() {
    println!("feature.rs");
    not_extra();
    very_basic();
    second()
}

#[cfg(not(feature = "japanese"))]
pub fn hello() -> &'static str {
    "hello"
}
#[cfg(feature = "japanese")]
pub fn hello() -> &'static str {
    "こんにちは"
}

#[cfg(feature = "basic")]
pub fn basic() {
    println!("basic");
}

#[cfg(feature = "extra")]
pub fn extra() {
    println!("extra");
}

#[cfg(not(feature = "extra"))]
pub fn not_extra() {
    println!("not extra");
}

#[cfg(not(any(feature = "basic", feature = "extra")))]
pub fn very_basic() {
    println!("very basic");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello(), "hello");
    }
}

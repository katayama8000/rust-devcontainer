pub fn run() {
    println!(
        "{}{}{}ok_or_else.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );
    let val: Option<i32> = Some(10);
    let val2: Option<i32> = None;

    let ret = val.ok_or_else(|| "No value".to_string());
    let _ret2: Result<_, String> = val2.ok_or_else(|| "No value".to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ok_or_else() {
        let val: Option<i32> = Some(10);
        let val2: Option<i32> = None;

        let ret: Result<i32, String> = val.ok_or_else(|| "No value".to_string());
        let _ret2: Result<i32, String> = val2.ok_or_else(|| "No value".to_string());

        assert_eq!(ret, Ok(10));
        assert_eq!(_ret2, Err("No value".to_string()));
    }

    #[test]
    fn test_ok_or() {
        let val: Option<i32> = Some(10);
        let val2: Option<i32> = None;

        let ret: Result<i32, String> = val.ok_or("No value".to_string());
        let _ret2: Result<i32, String> = val2.ok_or("No value".to_string());

        assert_eq!(ret, Ok(10));
        assert_eq!(_ret2, Err("No value".to_string()));
    }
}

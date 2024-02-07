pub fn run() {
    println!(
        "{}{}{}transpose.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let val = func().map(|x| x * 2);
    let val2: Option<Result<i32, String>> = Some(val);
    let val2: Result<Option<i32>, String> = val2.transpose();
    println!("{:?}", val2);
    // いつ使うか
    // mapの中でResultを返すときに、Option<Result<T, E>>になりがちなのでそういう時
    // 例
    let arr: Vec<i32> = vec![1, 2, 3];
    let val: Option<Result<i32, String>> = arr.last().map(|x| f(x));
    let val: Result<Option<i32>, String> = val.transpose();
}

fn f(val: &i32) -> Result<i32, String> {
    if *val == 1 {
        Ok(10)
    } else {
        Err("No value".to_string())
    }
}

fn func() -> Result<i32, String> {
    let val = Some(10);
    val.ok_or("No value".to_string())
}

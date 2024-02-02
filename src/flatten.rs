pub fn run() {
    println!("{}{}{}flatten.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", v);
    let flattened: Vec<i32> = v.into_iter().flatten().collect();
    println!("{:?}", flattened);
    let v = vec![Some(1), None, Some(3)];
    println!("{:?}", v);
    let flattened: Vec<i32> = v.into_iter().flatten().collect();
    println!("{:?}", flattened);
    let v = Some(Some(1));
    println!("{:?}", v);
    let flattened: Option<i32> = v.flatten();
    println!("{:?}", flattened);
}

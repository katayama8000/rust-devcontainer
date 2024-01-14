pub fn run() {
    println!(
        "{}{}{}stack_heap.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let mut v3 = vec![1, 2, 3];
    println!("v1: {:p}", &v1);
    println!("v2: {:p}", &v2);
    println!("v3: {:p}", &v3);
}

pub fn run() {
    println!("closure.rs");
    let x = 4;
    let equal_to_x = |z: i32| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    let outer_var = 42;
    // NG
    // fn function(i: i32) -> i32 {
    //     i + outer_var
    // }

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    closure_annotated(1);
}

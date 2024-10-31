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
    let vec_num = vec![1, 2, 3];
    let val = impl_1(vec_num);
    println!("{}", val());
    let closure = |i: i32| i + 1;
    where_1(closure);

    let fun = func;
}

fn impl_1(vals: Vec<i32>) -> impl Fn() -> i32 {
    move || vals.iter().sum()
}

fn where_1<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    f(1);
}

fn func() -> i32 {
    42
}

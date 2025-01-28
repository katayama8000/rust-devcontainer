use std::{thread, time::Duration};

pub fn run() {
    println!("closure.rs");
    let x = 4;
    let equal_to_x = |z: i32| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    let outer_var = 42;
    // NG
    // 外側の変数をキャプチャできないため
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

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let v = 10.0;

    {
        let equal_to_x = |z: f64| z == v;
    }

    let y = 4;
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

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
#[should_panic]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

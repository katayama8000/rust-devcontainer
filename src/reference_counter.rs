use std::rc::Rc;
// use BoxList::{Cons, Nil};
use RcList::{Cons, Nil};

pub fn run() {
    println!(
        "{}{}{}reference_counter.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );
    let s = Rc::new("hello");
    let t = s.clone();
    let mut u = s.clone();

    // count
    println!("count: {}", Rc::strong_count(&s));
    // eztract value
    println!("s: {}", *s);
    println!("t: {}", *t);

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // count
    println!("count: {}", Rc::strong_count(&a));
    // extract value
    println!("a: {:?}", a);

    let val1 = Rc::new(1);
    println!("count: {}", Rc::strong_count(&val1));
    let val2 = Rc::clone(&val1);
    println!("count: {}", Rc::strong_count(&val1));
    {
        let val3 = Rc::clone(&val1);
        println!("count: {}", Rc::strong_count(&val1));
    }
    println!("count: {}", Rc::strong_count(&val1));
}

// enum BoxList {
//     Cons(i32, Box<BoxList>),
//     Nil,
// }

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

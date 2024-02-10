use std::rc::Rc;

pub fn run() {
    println!("{}{}{}rc_arc.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let val = Rc::new(10);
    let val2 = val.clone();
    let val3 = val.clone();
    println!("val: {},", Rc::strong_count(&val));
    // reference counter is 3
    println!("val: {}", *val);
    // val is 10

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
    println!("{:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

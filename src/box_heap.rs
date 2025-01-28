use std::ops::Deref;

pub fn run() {
    println!(
        "{}{}{}box_heap.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    let byte_array = Box::new(byte_array);
    // print pointer address
    println!("byte_array: {:p}", byte_array);
    print_box(byte_array);

    let list = List::Cons(
        2,
        Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
// error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
// fn print_u8(s: [u8]) {
//     println!("s: {:?}", s);
// }

// fn print_u16(s: [u16]) {
//     println!("s: {:?}", s);
// }

fn print_box(s: Box<[u8]>) {
    println!("s: {:?}", s);
}

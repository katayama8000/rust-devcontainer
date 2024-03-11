pub fn run() {
    println!("display.rs");
    let str = "hello";
    let string = str.to_string();
    let my_struct = MyStruct { a: 1, b: 2 };
    let my_struct_string = my_struct.to_string();
    println!("my_struct_string: {}", my_struct_string);
}

struct MyStruct {
    a: i32,
    b: i32,
}

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

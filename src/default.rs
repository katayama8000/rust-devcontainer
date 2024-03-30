pub fn run() {
    println!("default.rs");
    let my_struct = MyStruct::default();
    println!("{:?}", my_struct);
    let my_struct2 = MyStruct2::default();
    println!("{:?}", my_struct2);
    let client_options = ClientOptions::default();
    println!("{:?}", client_options);
    let client = Client::new(ClientOptions {
        name: Some("John".to_string()),
        ..Default::default()
    });
    println!("{:?}", client);
}

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: i32,
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { a: 1, b: 2 }
    }
}

#[derive(Default, Debug)]
struct MyStruct2 {
    a: String,
    b: f32,
}

#[derive(Default, Debug)]
struct ClientOptions {
    name: Option<String>,
    age: Option<i32>,
}

#[derive(Debug)]
struct Client {
    name: String,
    age: i32,
}

impl Client {
    fn new(options: ClientOptions) -> Self {
        Self {
            name: options.name.unwrap_or("name".to_string()),
            age: options.age.unwrap_or(20),
        }
    }
}

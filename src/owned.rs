pub fn run() {
    println!("owned.rs");

    let name = "John";
    let own = name.to_owned();
    let own = name.to_string();

    let person = &Person {
        name: "John".to_string(),
    };

    let own = person.to_owned();
}

#[derive(Debug)]
struct Person {
    name: String,
}

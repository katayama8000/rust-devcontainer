pub fn run() {
    println!("into.rs");

    let name = "John";
    let person = Person::from(name);
    // into
    let person: Person = name.into();
    let name: &str = name.into();

    let animal = Animal {
        name: "cat".to_string(),
    };

    let name: String = animal.into();

    sample("hello");
    sample("world".to_string());
}

struct Person {
    name: String,
}

impl From<&str> for Person {
    fn from(name: &str) -> Self {
        Person {
            name: name.to_string(),
        }
    }
}

struct Animal {
    name: String,
}

impl Into<String> for Animal {
    fn into(self) -> String {
        self.name
    }
}

fn sample(v: impl Into<String>) {
    let s: String = v.into();
    println!("{}", s);
}

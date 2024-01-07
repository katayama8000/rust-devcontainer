use std::str::FromStr;

pub fn run() {
    println!("{}{}{}traits.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    Person::new("John", 25);
    let person = Person::from_str("mike")
        .map_err(|e| println!("Error: {:?}", e))
        .expect("person not found");

    let language = Language::English;
    let greet = person.greet(language);
    println!("greet: {}", greet);
}

#[derive(Debug)]
enum CustomError {
    NotFound,
}

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl FromStr for Person {
    type Err = CustomError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(CustomError::NotFound);
        }
        let name = s.to_string();
        let age = 10;

        Ok(Self { name, age })
    }
}

impl TryFrom<&str> for Person {
    type Error = CustomError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CustomError::NotFound);
        }
        let name = value.to_string();
        let age = 10;

        Ok(Self { name, age })
    }
}

enum Language {
    English,
    Spanish,
    Japanese,
}

trait Greet {
    fn greet(&self, language: Language) -> String;
}

impl Greet for Person {
    fn greet(&self, language: Language) -> String {
        match language {
            Language::English => "Hello".to_string(),
            Language::Spanish => "Hola".to_string(),
            Language::Japanese => "こんにちは".to_string(),
        }
    }
}

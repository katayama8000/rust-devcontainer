pub fn run() {
    println!("{}{}{}from.rs.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let p = Person::from("John,20");
    println!("Person: {:?}", p);
    let p = Person::from("Jane,23".to_string());
    println!("Person: {:?}", p);
    let s = String::from(&p);
    println!("String: {:?}", s);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        Person {
            name: parts.next().unwrap().to_string(),
            age: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl From<String> for Person {
    fn from(s: String) -> Self {
        let mut parts = s.split(',');
        Person {
            name: parts.next().unwrap().to_string(),
            age: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl From<&Person> for String {
    fn from(p: &Person) -> Self {
        format!("{},{}", p.name, p.age)
    }
}

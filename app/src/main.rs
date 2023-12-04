
// from と try_from

use std::convert::TryFrom;

struct Person {
    name: String,
    age: i32,
}

impl From <&str> for Person {
    fn from(s: &str) -> Self {
        let mut split = s.split(",");
        Person {
            name: split.next().unwrap().to_string(),
            age: split.next().unwrap().parse().unwrap(),
        }
    }
}

impl TryFrom<&i32> for Person {
    type Error = String;
    fn try_from(age: &i32) -> Result<Self, Self::Error> {
        if *age < 0 {
            Err("年齢は正の数である必要があります".to_string())
        } else {
            Ok(Person {
                name: "Taro".to_string(),
                age: *age,
            })
        }
    }
}
fn main() {
    println!("Hello, world!");
    let p = Person::from("Taro,20");
    println!("name: {}, age: {}", p.name, p.age);

    let p = Person::try_from(&20);
    match p {
        Ok(p) => println!("name: {}, age: {}", p.name, p.age),
        Err(e) => println!("Error: {}", e),
    }

}

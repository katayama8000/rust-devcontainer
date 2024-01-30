pub fn run() {
    println!("{}{}{}as_ref.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let p = Person {
        name: "John".to_string(),
        age: 20,
    };

    let s = p.as_ref();
    println!("s: {:?}", s);

    // let s = "Jane".to_string();
    // let s_ref = s.as_ref();
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl AsRef<str> for Person {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

// impl AsRef<String> for Person {
//     fn as_ref(&self) -> &String {
//         &self.name
//     }
// }

// impl AsRef<[u8]> for Person {
//     fn as_ref(&self) -> &[u8] {
//         self.name.as_bytes()
//     }
// }

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_ref() {
        let p = Person {
            name: "John".to_string(),
            age: 20,
        };

        assert_eq!("John", p.as_ref());
    }
}

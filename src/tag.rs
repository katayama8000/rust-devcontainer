use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn run() {
    println!("tag.rs");
    let animal = Animal::Dog {
        name: "Choco".to_string(),
        dog_type: "Toy Poodle".to_string(),
    };
    println!("{:?}", animal);

    // serialize
    let json = serde_json::to_string(&animal).unwrap();
    println!("{}", json);

    // deserialize
    let animal: Animal = serde_json::from_str(&json).unwrap();
    println!("{:?}", animal);

    let animal_json = r#"{"type":"dog","name":"Choco","dog_type":"Toy Poodle"}"#;

    let animal: Animal = serde_json::from_str(animal_json).unwrap();
    println!("{:?}!!!!!", animal);
}

// try serde tag
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum Animal {
    Dog { name: String, dog_type: String },
    Cat { name: String, cat_type: String },
    Rabbit { name: String, rabbit_type: String },
    Bird { name: String, bird_type: String },
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let animal_json = r#"{"type":"dog","name":"Choco","dog_type":"Toy Poodle"}"#;
        let animal: Animal = serde_json::from_str(animal_json)?;
        println!("{:?}", animal);
        assert_eq!(
            animal,
            Animal::Dog {
                name: "Choco".to_string(),
                dog_type: "Toy Poodle".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let animal = Animal::Dog {
            name: "Choco".to_string(),
            dog_type: "Toy Poodle".to_string(),
        };
        let json = serde_json::to_string(&animal)?;
        println!("{}", json);
        assert_eq!(
            json,
            r#"{"type":"dog","name":"Choco","dog_type":"Toy Poodle"}"#
        );
        Ok(())
    }
}

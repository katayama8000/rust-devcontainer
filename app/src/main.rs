use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data(pub Vec<Item>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "number")]
    Number { x: i64 },
    #[serde(rename = "string")]
    String { x: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"
        [
            { "type": "string", "x": "abc" },
            { "type": "number", "x": 100 }
        ]
    "#;

    let data: Data = serde_json::from_str(json)?;
    println!("{:?}", data);
    println!("{:?}", data.0);
    for item in data.0 {
        match item {
            Item::Number { x } => println!("x is Number({})", x),
            Item::String { x } => println!("x is String({})", x),
        }
    }

    Ok(())
}

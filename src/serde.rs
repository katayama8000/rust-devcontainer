use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn run() {
    println!("{}{}{}serde.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let val = [("data".to_string())];
    let serialized = serde_json::to_value(&val).unwrap();
    println!("serialized = {}", serialized);

    #[derive(Serialize, Deserialize)]
    struct Human {
        name: String,
        age: i32,
    }

    #[derive(Serialize, Deserialize)]
    struct Family {
        humans: Vec<Human>,
        adress: String,
    }

    let humans = vec![
        Human {
            name: "Alice".to_string(),
            age: 42,
        },
        Human {
            name: "Bob".to_string(),
            age: 42,
        },
    ];

    let family = Family {
        humans,
        adress: "Tokyo".to_string(),
    };

    let serialized = serde_json::to_value(&family).unwrap();
    println!("serialized = {}", serialized);

    #[derive(Serialize)]
    struct DoctorConsultationComment {
        comment_id: String,
        consultation_id: String,
        facility_id: String,
        issue_id: String,
    }

    let comment = DoctorConsultationComment {
        comment_id: String::from("Aagdh9GHmJK0MqO8e7kr"),
        consultation_id: String::from("XHsiqWGYX7iZJ3eD2dd6"),
        facility_id: String::from("lzGRC5I0V2dia0MmrtBs2kv98GC3"),
        issue_id: String::from("CJpgnSr5sDgK4K7cttys"),
    };

    // JSON文字列に変換
    let json = serde_json::to_string(&comment).unwrap();
    println!("JSON: {}", json);
}

fn to_value<T>(value: &T) -> Value
where
    T: Serialize,
{
    serde_json::to_value(value).expect("msg")
}
//test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let val = [("data".to_string())];
        let serialized = to_value(&val);
        assert_eq!(
            serialized,
            Value::Array(vec![Value::String("data".to_string())])
        );
    }
}

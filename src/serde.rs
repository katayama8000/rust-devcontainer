use serde::Serialize;
use serde_json::Value;

pub fn run() {
    println!("{}{}{}serde.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let val = [("data".to_string())];
    let serialized = serde_json::to_value(&val).unwrap();
    println!("serialized = {}", serialized);
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

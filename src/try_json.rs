use serde::Serialize;
use serde_json::json;

pub fn run() {
    println!("try_json.rs");
    let val1 = serde_json::json!(null);
    println!("val1: {}", val1);
    let val2 = serde_json::json!(true);
    println!("val2: {}", val2);
    let val3 = serde_json::json!("string");
    println!("val3: {}", val3);
    let val4 = serde_json::json!(42);
    println!("val4: {}", val4);
    let val5 = serde_json::json!({
        "key1": "value1",
        "key2": "value2",
    });
    println!("val5: {}", val5);
    let val6 = serde_json::json!([1, 2, 3]);
    println!("val6: {}", val6);
    let val7 = serde_json::json!({
        "key1": {
            "key2": {
                "key3": "value3",
            },
        },
    });

    println!("val7: {}", val7);
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };
    // tovalue
    let arr = [1, 2, 3];
    let val8 = serde_json::to_value(arr);
    println!("val8: {:?}", val8);
    let result_val = serde_json::json!(Some(10));
    println!("result_val: {}", result_val);
    let result_val = serde_json::json!(Option::<i32>::None);
    println!("result_val: {}", result_val);
    let val8 = serde_json::to_value(person);
    println!("val8: {:?}", val8);

    let mut val9 = json!({
        "name": "John",
        "age": 30,
    });
    println!("val9: {}", val9);

    val9["country"] = json!("Japan");
    println!("val9: {}", val9);

    let ichiro = Human {
        my_no: 1,
        name: "ichiro".into(),
    };
    let maybe_ichiro = Option::Some(&ichiro);

    let r1 = maybe_ichiro.clone(); // Option<&Human>
    let r2 = maybe_ichiro.cloned(); // Option<Human>
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Clone, Debug)]
struct Human {
    my_no: i32,
    name: String,
}

use std::{collections::HashMap, vec};

pub fn run() {
    println!("{}{}{}map.rs{}{}{}", "🦀", "🦀", "🦀", "🦀", "🦀", "🦀");
    let arr = [1, 2, 3];
    let arr2 = arr.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("arr2: {:?}", arr2);

    let arr = [1, 2, 3];
    // make Person from i32
    let arr2 = arr.iter().map(|x| Person::from(*x)).collect::<Vec<_>>();
    let arr3 = arr.into_iter().map(Person::from).collect::<Vec<_>>();
    println!("arr2: {:?}", arr2);

    let arr = [1, 2, 3];
    // make Person from i32
    // let arr2 = arr
    //     .iter()
    //     .map(|x: &i32| Person::new(*x))
    //     .collect::<Vec<_>>();
    // println!("arr2: {:?}", arr2);

    assert_eq!(arr2, arr3);

    let arr: Vec<Person> = vec![
        Person::new("John".to_string(), 20),
        Person::new("John".to_string(), 30),
        Person::new("John".to_string(), 40),
    ];
    // make Person from i32
    // let arr2 = arr.into_iter().map(Person::new).collect::<Vec<_>>();
    // TODO: why work?
    let arr: Vec<Person> = arr.into_iter().map(Person::new2).collect();
    println!("arr: {:?}", arr);

    let v = Some(10);
    let v2 = v.map(|x| x);
    let mut hash_map: HashMap<usize, usize> = HashMap::new();
    hash_map.insert(1, 2);
    hash_map.insert(2, 3);
    let val = hash_map.get(&1).map(|x| x * 2);
    println!("val: {:?}", val);
    let val = hash_map.entry(1).or_insert(3);

    println!("-----");
    let v: Result<i32, String> = Ok(10);
    let v2 = v.clone().map(|x| x);
    // and_then
    let v_clone = v.clone();
    let v2 = v.and_then(|x| Ok(x));

    let ret = errable_add(10, 20);
    let ret = ret.map(|x| {
        println!("x: {}", x);
        x
    });
    println!("ret: {:?}", ret);
    // and_then
    let ret = errable_add(10, 20);
    let ret = ret.and_then(|x| {
        println!("x: {}", x);
        Ok(x)
    });
    let optipn_num = Some(10);
    let ret: Result<i32, String> = optipn_num.ok_or_else(|| "error".to_string());
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

impl From<i32> for Person {
    fn from(s: i32) -> Self {
        Person {
            name: "John".to_string(),
            age: s as u8,
        }
    }
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn new2(person: Person) -> Self {
        Person {
            name: person.name,
            age: person.age,
        }
    }
}

fn errable_add(a: i32, b: i32) -> Result<i32, String> {
    if a + b > 0 {
        Ok(a + b)
    } else {
        Err("sum is negative".to_string())
    }
}

#[derive(Debug)]
pub struct AppleError(String);

#[derive(Debug)]
pub struct BananaError(String);

#[derive(Debug)]
pub enum FruitError {
    AppleErrorCase(AppleError),
    BananaErrorCase(BananaError),
}

#[derive(Debug)]
pub struct Apple(String);
#[derive(Debug)]
pub struct Bananas(String);
#[derive(Debug)]
pub struct Cherries(String);

fn function_a(apple: Apple) -> Result<Bananas, AppleError> {
    Ok(Bananas("Banana".to_string()))
}

fn function_b(banana: Bananas) -> Result<Cherries, BananaError> {
    Ok(Cherries("Cherry".to_string()))
}

fn function_a_with_fruit_error(apple: Apple) -> Result<Bananas, FruitError> {
    function_a(apple).map_err(|apple_error| FruitError::AppleErrorCase(apple_error))
}

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
    let arr2 = arr
        .iter()
        .map(|x: &i32| Person::new(*x))
        .collect::<Vec<_>>();
    println!("arr2: {:?}", arr2);

    assert_eq!(arr2, arr3);

    let arr = [1, 2, 3];
    // make Person from i32
    let arr2 = arr.into_iter().map(Person::new).collect::<Vec<_>>();
    // TODO: why work?
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
    fn new(name: i32) -> Self {
        Person {
            name: "John".to_string(),
            age: name as u8,
        }
    }
}

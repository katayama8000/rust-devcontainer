pub fn run() {
    let str = String::from("Hello");
    let result1: &str = str.as_ref();
    println!("{:?}", result1);
    let result2: &[u8] = str.as_ref();
    println!("{:?}", result2);

    let person: Person<i64> = 20.into();
    println!("{:?}", person);

    let person = Person { age: 32 };
    let age: i32 = person.into();

    let mut h = Height(170);
    let str = String::from("Hello");
    func(str);
}

#[derive(Debug)]
struct Person<T> {
    age: T,
}

impl Into<Person<i32>> for i32 {
    fn into(self) -> Person<i32> {
        println!("i32");
        Person { age: self }
    }
}

impl Into<Person<i64>> for i64 {
    fn into(self) -> Person<i64> {
        println!("i64");
        Person { age: self + 100 }
    }
}

impl Into<i32> for Person<i32> {
    fn into(self) -> i32 {
        self.age
    }
}

impl Into<i64> for Person<i32> {
    fn into(self) -> i64 {
        self.age as i64
    }
}

struct Height(i64);
struct Width(i64);

fn func(s: String) {
    s.bow();
}

trait StringExt {
    fn bow(&self) -> String;
}

impl StringExt for String {
    fn bow(&self) -> String {
        format!("{} bow", self)
    }
}

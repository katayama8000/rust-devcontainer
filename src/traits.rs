use std::future::Future;

pub fn run() {
    println!("traits.rs");
    let person = Person {
        name: "Alice".to_string(),
    };

    println!("{}", person.greeting());
}

trait Hello1 {
    fn hello1(&self) -> String;
}

trait Hello2 {
    fn hello2(&self) -> String;
}

trait Hello3 {
    fn hello3(&self) -> String;
}

trait Greeting
where
    Self: Hello1 + Hello2 + Hello3,
{
    fn greeting(&self) -> String {
        format!("{} {} {}", self.hello1(), self.hello2(), self.hello3())
    }
}

struct Person {
    name: String,
}

impl Hello1 for Person {
    fn hello1(&self) -> String {
        format!("Hello1, {}", self.name)
    }
}

impl Hello2 for Person {
    fn hello2(&self) -> String {
        format!("Hello2, {}", self.name)
    }
}

impl Hello3 for Person {
    fn hello3(&self) -> String {
        format!("Hello3, {}", self.name)
    }
}

impl Greeting for Person {}

trait FutureTrait {
    async fn hello(&self) -> String;
    fn hello2(&self) -> impl Future<Output = String>;
}

impl FutureTrait for Person {
    async fn hello(&self) -> String {
        format!("Hello, {}", self.name)
    }

    fn hello2(&self) -> impl Future<Output = String> {
        async { format!("Hello2, {}", self.name) }
    }
}

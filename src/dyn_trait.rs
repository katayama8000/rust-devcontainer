struct Dove;
struct Duck;

trait Tweet {
    fn tweet(&self);
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }
    fn shout(&self) {
        println!("Uooooooooooooh!");
    }
}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    // インスタンスのメソッドシグネチャ
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
// `Sheep`に`Animal`トレイトを実装する。
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
// `Cow`に`Animal`トレイトを実装する。
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// Animalを実装した何らかの構造体を返す。
// ただし、コンパイル時にはどの実装か分からない。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn return_animal() -> Box<Sheep> {
    Box::new(Sheep {})
}

trait Foo {
    fn method(&self) -> String;
}

struct Bar;

impl Foo for Bar {
    fn method(&self) -> String {
        "Bar".to_string()
    }
}

fn do_something() -> Box<dyn Foo> {
    Box::new(Bar {})
}

fn return_bar() -> Box<Bar> {
    Box::new(Bar {})
}

pub fn run() {
    println!(
        "{}{}{}dyn_trait.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let dove = Dove {};
    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];

    for bird in bird_vec {
        bird.tweet();
    }

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

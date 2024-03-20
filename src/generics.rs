pub fn run() {
    println!("generics.rs",);
    let p = Point { x: 1, y: 2 };
    println!("p.x: {}", p.get_x());
    let p2 = Point2 { x: 1, y: 2.0 };
    notify(p2);
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

trait Summary2 {
    fn summarize_author(&self) -> String;
}

impl Summary for Point<i32> {
    fn summarize(&self) -> String {
        format!("Point: x={}, y={}", self.x, self.y)
    }
}

impl Summary2 for Point<f32> {
    fn summarize_author(&self) -> String {
        format!("Point: x={}, y={}", self.x, self.y)
    }
}

impl Summary for Point2<i32, f32> {
    fn summarize(&self) -> String {
        format!("Point2: x={}, y={}", self.x, self.y)
    }
}

impl Summary2 for Point2<i32, f32> {
    fn summarize_author(&self) -> String {
        format!("Point2: x={}, y={}", self.x, self.y)
    }
}

fn notify<T>(item: T)
where
    T: Summary + Summary2,
{
    println!("notify: {}", item.summarize());
}

pub fn run() {
    println!(
        "{}{}{}generics.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );
    let point1 = Point { x: 1, y: 2.0 };
    let point2 = Point { x: "hello", y: "a" };
    let point3 = point1.mix_up(point2);
    println!("point3.x: {}, point3.y: {}", point3.x, point3.y);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

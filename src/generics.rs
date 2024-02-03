pub fn run() {
    println!(
        "{}{}{}generics.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );
    let p = Point { x: 5, y: 10.0 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
    let line = Line {
        start: Point { x: 3, y: 4.0 },
        end: Point { x: 10, y: 15.0 },
    };
    println!("line.start.x = {}", line.start.x);
    println!("line.start.y = {}", line.start.y);
    println!("line.end.x = {}", line.end.x);
    println!("line.end.y = {}", line.end.y);
    let v = Val { val: 3.0 };
    println!("val = {}", v.value());
    let v = GenVal {
        gen_val: Circle { radius: 3.0 },
    };
    v.show();

    let v = GenVal::new(Circle { radius: 3.0 }); // Fix: Call new method correctly
    v.show();
}

struct Point<T, Y> {
    x: T,
    y: Y,
}

struct Line<T, Y>
where
    T: Shape,
    Y: Shape,
{
    start: T,
    end: Y,
}

trait Shape {
    fn area(&self) -> f64;
}

trait Shape2 {
    fn area(&self) -> String;
}

impl Shape for Point<i32, f64> {
    fn area(&self) -> f64 {
        (self.x as f64) * self.y
    }
}

struct Circle {
    radius: f64,
}

impl Shape2 for Circle {
    fn area(&self) -> String {
        format!("Circle area: {}", 3.14 * self.radius * self.radius)
    }
}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
// Valに対してimpl
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T>
where
    T: Shape2,
{
    fn show(&self) {
        println!("show: {}", self.gen_val.area());
    }

    fn new(v: T) -> GenVal<T> {
        GenVal { gen_val: v }
    }
}

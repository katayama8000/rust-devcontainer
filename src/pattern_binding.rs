pub fn run() {
    let numbers = vec![0, 1, 2, 3, 5, 8, 13, 21];
    for number in numbers {
        match number {
            fib @ 0 | fib @ 1 => {
                println!("Base case: {}", fib);
            }
            fib @ 2..=10 => {
                println!("Small Fibonacci: {}", fib);
            }
            fib => {
                println!("Large Fibonacci: {}", fib);
            }
        }
    }
}

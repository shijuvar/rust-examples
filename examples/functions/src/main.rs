fn main() {
    hello();
    let a = 10.0;
    let b = 4.0;
    println!("{a} + {b} = {}", add(a, b));
    println!("{a} - {b} = {}", subtract(a, b));
    println!("{a} * {b} = {}", multiply(a, b));
    println!("{a} / {b} = {}", divide(a, b));

    // return a tuple of return values
    let result = swap(200, 300);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (x, y) = swap(result.0, result.1);
    println!("{} {}", x, y);

    // Destructure the returned tuple into two variables
    let (total, difference) = calculate(100, 60);
    println!("Sum: {}, Difference: {}", total, difference);
}

fn hello() {
    println!("Hello, world!");
}
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x); // Explicit return
}
// Define the return type as a tuple: (i32, i32)
fn calculate(a: i32, b: i32) -> (i32, i32) {
    let sum = a + b;
    let diff = a - b;
    (sum, diff) // Implicit return (no semicolon)
}

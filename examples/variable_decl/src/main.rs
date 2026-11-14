const PI: f64 = 3.14;

fn main() {
    // let x = 5; // immutable
    let mut x = 5; // mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of PI is: {PI}");

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    // Compound Types
    // Rust has two primitive compound types: tuples and arrays.
    let tup: (i32, f64, u8) = (900, 99.5, 1);
    println!("The value of tup is: {:?}", tup);
    let (a, b, c) = tup;

    println!("The value of a is {a}, b is: {b},  c is: {c}");

    let mut z: (i32, f64, u8) = (500, 55.55, 10);
    z.0 = 1000;

    let num1 = z.0;

    let num2 = z.1;

    let num3 = z.2;
    println!("The value of num1 is: {num1}, num2 is: {num2}, num3 is: {num3}");
}

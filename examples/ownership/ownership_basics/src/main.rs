fn main() {
    // Point 1: scope
    {
        let s = String::from("Hello World!"); // s is valid from this point forward
        println!("{s}" )

    } // this scope is now over, and s is no longer valid
    // When a variable goes out of scope, Rust calls a special function called drop,

    // println!("{s}" ) // this will be an error because s is no longer valid

    // Point 2: copy of simple values
    // integers are simple values with a known, fixed size and that values are pushed onto the stack.
    let x = 100;
    let  y = x;
    println!("x: {}, y: {}", x, y);

    // Point 3: move of complex values
    let s1 = String::from("Rustacean");
    let s2 = s1; // Value s1 is moved to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This will be an error because s1 is no longer valid

    // Point 4: cloning of complex values
    let s3 = s2.clone(); // Value s2 is cloned to s3
    println!("s2: {}, s3: {}", s2, s3);

    // Point 5: passing a complex value to a function return of it
    let (s4, len) = calculate_length(s3);
    println!("s4: {}, len: {}", s4, len);
    // println!("s3: {}", s3); // This will be an error because s3 is no longer valid

    // Point 6: passing a simple value to a function return of it
    let  x=200;
    let y=copy_integer(x);
    println!("x: {}, y: {}", x, y);

    // Point 7: reference borrowing: passing a reference to a complex value to a function
    let len= calculate_length_borrow(&s4); // ampersand represents references
    println!("s5: {}, length: {}", s4, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// function that has a reference to an object as a parameter instead of taking ownership of the value
fn calculate_length_borrow(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

fn copy_integer(x: i32) -> i32 {
    x+100
}
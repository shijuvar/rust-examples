/*
    Rules for mutable references:
    It's not possible to have multiple mutable references to the same value.
    It's possible to have multiple immutable references to the same value.
 */
fn main() {
    let mut s = String::from("Rustacean");

    // Mutable references are having one restriction:
    // If you have a mutable reference to a value, you can have no other references to that value
    let r1 = &mut s;
    // let r2 = &mut s; // uncomment to see the error: cannot borrow `s` as mutable more than once at a time
    // println!("r1: {r1}, r2: {r2}"); // uncomment to see the error


    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;

    let mut s = String::from("Rustacean");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {r1}, r2:{r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("r3: {r3}");
}


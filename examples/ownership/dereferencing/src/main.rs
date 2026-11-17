// ref: https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html
fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    println!("a = {}", a);
    *x += 1;                 // *x on the left-side modifies the heap value,
    //     so x points to the value 2
   println!("x = {}", *x);
    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    println!("r1 = {:?}", r1);
    let b: i32 = **r1;       // two dereferences get us to the heap value
    println!("b = {}", b);
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    println!("r2 = {:?}", r2);
    let c: i32 = *r2;    //}
    println!("c = {}", c);
}

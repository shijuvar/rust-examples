/*
    &T: immutable (shared) reference
    &mut T: mutable (exclusive) reference
    *const T: immutable raw pointer
    *mut T: mutable raw pointer

    references and raw pointers can be dereferenced using * operator,
    but for raw pointers, it requires an unsafe block:
 */
fn main() {

    let mut x: u32 = 100;

    let ref1: &u32 = &x;
    println!("ref1 value: {}", *ref1);
    let raw1: *const u32 = &x;
    println!("raw1 address: {:p}", raw1);
    // SAFETY: raw1 points to the still-live, aligned x and is only read here.
    println!("raw1 value: {}", unsafe { *raw1 }); // raw pointers require an unsafe block

    let ref2: &mut u32 = &mut x;
    println!("ref2 value: {}", *ref2);
    let raw2: *mut u32 = &mut x;
    println!("raw2 address: {:p}", raw2);
    // SAFETY: raw2 points to the still-live, aligned x and is only read here.
    println!("raw2 value: {}", unsafe { *raw2 });

    // Box<T> is one of the smart pointers
    /*
      smart pointers are data structures that act like pointers but contain additional metadata
       and capabilities, such as managing heap memory or tracking ownership.
       Unlike standard references (& and &mut) that only borrow data,
       smart pointers usually own the data they point to
     */
    let x: Box<u32> = Box::new(100);
    
    let y: &u32 = &*x;
    println!("y value: {}", y);

    let x: u32 = 100;

    let y = x;           // y: u32, a copy of x
    // ref is to get a variable of the reference type instead of a value.
    let ref z = x;       // z: &u32, points to x
    println!("z address: {:p}, value: {}", z, *z);
    let mut x:  u32=100;
    let ref mut zz = x;  // zz: &mut u32, points to x
    println!("zz address: {:p}, value: {}", zz, *zz);

}

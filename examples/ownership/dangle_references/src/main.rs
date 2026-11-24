fn main() {
    let text = no_dangle();
    println!("{text}");
}

// This would create a dangling reference, so Rust rejects it:
// fn dangle() -> &String {
//     let text = String::from("hello");
//     &text // `text` is dropped when the function returns
// }

// Return ownership instead, so the string stays alive in the caller.
fn no_dangle() -> String {
    String::from("hello")
}

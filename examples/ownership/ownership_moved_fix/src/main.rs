fn main() {
    let first_name = String::from("Shiju");
    // All heap data must be owned by exactly one variable.
    // clone() returns a duplicate of the value.
    let full_name = add_lastname(first_name.clone());
    println!("full name: {full_name}, first name: {first_name}");
}

fn add_lastname(mut name: String) -> String {
    name.push_str(" Varghese");
    name
}
fn main() {
    // string has been allocated on the heap. It is owned by first_name.
    let first_name = String::from("Shiju");
    // Calling add_lastname moves the ownership of the string from first_name to name.
    // The string data is not copied, but the pointer to the data is copied.
    // This function returned full_name, transferring ownership of the string to full_name.
    let full_name = add_lastname(first_name);
    println!("full name: {full_name}");
}

fn add_lastname(mut name: String) -> String {
    // resizes the string’s heap allocation.
    name.push_str(" Varghese");
    name
}

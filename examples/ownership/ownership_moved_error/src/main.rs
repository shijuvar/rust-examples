fn main() {
    let first_name = String::from("Shiju");
    let full_name = add_lastname(first_name);
    println!("full name: {full_name}, first name:{first}");
}


fn add_lastname(mut name: String) -> String {
    name.push_str(" Varghese");
    name
}

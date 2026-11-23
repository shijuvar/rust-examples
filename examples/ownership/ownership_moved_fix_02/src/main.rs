fn main() {
    let mut first_name = String::from("Shiju");
    println!("first name: {first_name}");
    let full_name = add_lastname(&mut first_name);
    println!("full name: {full_name}, first name: {first_name}");
}

fn add_lastname(name: &mut String) -> String {
    name.push_str(" Varghese");
    name.clone()
}
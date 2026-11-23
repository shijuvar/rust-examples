fn main() {
    let first_name = String::from("Shiju");
    let full_name = add_lastname(&first_name);
    println!("full name: {full_name}, first name: {first_name}");
}

fn add_lastname(name: &str) -> String {
    format!("{name} Varghese")
}

#[derive(Debug)]
struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    dob: String,
    email: String,
    location: String,
}

// Implementation block for Person
impl Person {
    /*
    &self is a short for self: &Self.
    Within an impl block, the type Self is an alias for the type that the impl block is for.
    Methods must have a parameter named self of type Self for their first parameter.
    Note that we still need to use the & in front of the self shorthand
    to indicate that this method borrows the Self instance.
     */
    fn print_name(&self) {
        println!("\n{} {}(id:{})", self.first_name, self.last_name, self.id);
    }

    fn print_details(&self) {
        println!(
            "[Date of Birth: {}, Email: {}, Location: {} ]",
            self.dob, self.email, self.location
        );
    }
    fn change_location(&mut self, location: String) {
        self.location = location;
    }
    // An associated function that doesn't have self as the first parameter
    fn greet_person(&self, msg: String) {
        println!("{}, {} {}!", msg, self.first_name, self.last_name);
    }
    //
    fn get_person(id: u32, first_name: String, last_name: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
            dob: String::new(),
            email: String::new(),
            location: String::new(),
        }
    }
}

// Rust allows multiple impl blocks for the same struct
impl Person {
    fn print(&self) {
        self.print_name();
        self.print_details();
    }
}
fn main() {
    let person1 = Person {
        id: 1,
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        dob: String::from("01/01/1970"),
        email: String::from("john.doe@example.com"),
        location: String::from("New York"),
    };
   person1.print();
   person1.greet_person(String::from("Hello"));

   let mut person2=Person::get_person(2, String::from("Shiju"), String::from("Varghese"));
    person2.email=String::from("shiju.varghese@example.com");
    person2.change_location(String::from("Kochi"));
   person2.print();
}
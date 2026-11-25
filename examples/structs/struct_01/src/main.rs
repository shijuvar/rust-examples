
#[derive(Debug)]
struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    dob: String,
    email: String,
    location: String,
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
    let mut person2 = Person {
        id: 2,
        first_name: String::from("Shiju"),
        last_name: String::from("Varghese"),
        dob: String::from("1979-02-17 00:00:00 +0000 UTC"),
        email: String::from("shiju@email.com"),
        location: String::from("Kochi"),
    };
    person2.email=  String::from("shijuv@email.com");
    let person3 = get_person(3, String::from("Rahul"), String::from("Krishnan"));
    println!(
        "Person 1: id={}, first_name={}, last_name={}, dob={}, email={}, location={}",
        person1.id, person1.first_name, person1.last_name, person1.dob, person1.email, person1.location
    );
    println!("Person 2: {:?}", person2);
    println!("Person 3: {:?}", person3);
}

fn get_person(id:  u32, first_name: String, last_name: String) -> Person {
    Person {
        id,
        first_name,
        last_name,
        dob: String::new(),
        email: String::new(),
        location: String::new(),
    }
}
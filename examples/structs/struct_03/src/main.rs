use chrono::NaiveDate;
#[derive(Default)]
struct Employee {
    first_name: String,
    last_name: String,
    dob: Option<NaiveDate>,
    job_title: String,
    location: String,
}

impl Employee {
    fn print_name(&self) {
        println!("\n{} {}", self.first_name, self.last_name);
    }

    fn print_details(&self) {
        let dob_display = match self.dob {
            Some(date) => format!("{} 00:00:00 +0000 UTC", date.format("%Y-%m-%d")),
            None => String::new(),
        };
        println!(
            "Date of Birth: {}, Job: {}, Location: {}",
            dob_display, self.job_title, self.location
        );
    }
}

#[derive(Default)]
struct Developer {
    employee: Employee,
    skills: Vec<String>,
}

impl Developer {
    fn print_name(&self) {
        self.employee.print_name();
    }

    fn print_details(&self) {
        self.employee.print_details();
    }
}

fn main() {
    let steve = Developer {
        employee: Employee {
            first_name: String::from("Steve"),
            last_name: String::from("John"),
            dob: NaiveDate::from_ymd_opt(1990, 2, 17),
            job_title: String::from("Software Engineer"),
            location: String::from("San Fancisco"),
        },
        skills: vec![
            String::from("Go"),
            String::from("Docker"),
            String::from("Kubernetes"),
        ],
    };
    steve.print_name();
    steve.print_details();

    let mut d = Developer::default();
    d.employee = Employee {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        ..Default::default()
    };
    d.skills = vec![String::from("Go"), String::from("Docker")];
    d.print_name();
    d.print_details();
}
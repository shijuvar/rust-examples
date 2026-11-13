use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses = 0;

    while guesses < 5 {
        println!("Please input your guess.");

        let mut guess = String::new();

        let bytes_read = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if bytes_read == 0 {
            println!("No input received.");
            return;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guesses += 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    println!("You have used all 5 guesses. The secret number was: {secret_number}");
}
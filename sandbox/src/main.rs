use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut random_number: i32;
    loop {
        println!("Generating random number...");
        random_number = rand::thread_rng().gen_range(0..1000);
        println!("Random number generated!");

        print!("Please enter your guess: ");
        let _ = io::stdout().flush();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read from user input");

        /* The user must press enter to satisfy read_line and input their guess,
        which adds a newline character to the string.
        For example, if the user types 5 and presses enter,
        guess looks like this: 5\n. The \n represents “newline.”
        (On Windows, pressing enter results in a carriage return and a newline, \r\n.)
        The trim method eliminates \n or \r\n, resulting in just 5. */

        if guess.trim() == "quit" {
            break;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("Congratulations! You guessed the secret number!");
                break;
            }

            Ordering::Greater => {
                println!("Missed! The number you guessed is greater than the secret number.")
            }
            Ordering::Less => {
                println!("Missed! The number you guessed is less than the secret number.")
            }
        }
    }

    println!("The secret number was: {}", random_number);
}

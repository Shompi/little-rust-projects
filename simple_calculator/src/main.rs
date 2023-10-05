use core::panic;
use std::io;
/**

   This simple calculator will show a menu with different operations
   - add
   - substract
   - multiply
   - divide

   and just take in 2 numbers.
*/

enum MyErrors {
    NotInteger,
}

fn print_menu() {
    print!("\nWelcome to SimpleCalculator!\n");
    print!("Select your choice:\n\n");
    print!("1) Add\n");
    print!("2) Substract\n");
    print!("3) Multiply\n");
    print!("4) Divide\n\n");
    print!("9) Exit application\n\n");
}

fn main() {
    // Main loop
    loop {
        print_menu();
        let mut input_choice = String::new();

        io::stdin()
            .read_line(&mut input_choice)
            .expect("Failed to read user input.\n");

        let choice_number: Result<i32, _> = input_choice.trim().parse();

        // No switch statement in rust so we use match pattern
        match choice_number {
            Ok(num) => match num {
                1 => add(),
                2 => substract(),
                3 => multiply(),
                4 => divide(),
                9 => break,
                _ => println!("The choice you have entered is not valid.\n"),
            },
            Err(_) => print!("There was an error parsing this number.\n"),
        }
    }

    println!("\nThe application has finished.");
}

fn add() {
    println!("Please input the first number: ");
    let num_a = parse_number();
    let num_b: Result<i32, MyErrors>;
    match num_a {
        Ok(parsed_a) => {
            println!("Please input the second number: ");
            num_b = parse_number();

            match num_b {
                Ok(parsed_b) => println!(
                    "The sum of {} + {} is: {}",
                    parsed_a,
                    parsed_b,
                    parsed_a + parsed_b
                ),
                Err(_) => println!("The second number you entered is couldn't be parsed.\n"),
            }
        }
        Err(_) => panic!("There was an error during execution of this program.\n"),
    }
}

fn substract() {
    println!("This operation is not yet implemented.")
}

fn multiply() {
    println!("This operation is not yet implemented.")
}

fn divide() {
    println!("This operation is not yet implemented.")
}

fn parse_number() -> Result<i32, MyErrors> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let number: Result<i32, _> = input.trim().parse();

            match number {
                Ok(num) => Ok(num),
                Err(_) => {
                    println!("There was an error parsing this integer.");
                    Err(MyErrors::NotInteger)
                }
            }
        }
        Err(_) => panic!("There was an error reading user input."),
    }
}

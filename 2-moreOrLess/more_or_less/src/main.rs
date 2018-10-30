extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
// Trait to get flush()
use std::io::Write;

const EXIT_COMMAND: &str = "exit";

fn main() {
    println!("Guess the secret number (between 0 and 100) !");
    println!("Type \"{}\" to stop the game", EXIT_COMMAND);

    let secret_number = rand::thread_rng().gen_range(0, 101);
    // println!("The secret number is: {}", secret_number);

    let mut tries = 1;

    loop {
        let guess = get_input_number();
        // println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("It's bigger."),
            Ordering::Greater => println!("It's smaller."),
            Ordering::Equal => {
                println!("You won in {} tries !", tries);
                break;
            }
        }

        tries += 1;
    }
}

/// Gets a number from standard input
fn get_input_number() -> u32 {
    loop {
        print!("Please input your guess : ");
        io::stdout().flush().expect("Exception while flushing standard output");
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess = guess.trim(); // Shadowing variable "guess"

                if guess.eq_ignore_ascii_case(EXIT_COMMAND) {
                    println!("Goodbye!");
                    std::process::exit(0);
                }

                match guess.parse() {
                    Ok(parsed_value) => {
                        return parsed_value;
                    }
                    Err(e) => println!("\"{}\" is not a valid number : {}", guess.trim(), e)
                }
            }
            Err(e) => println!("Error : {}", e)
        }
    }
}
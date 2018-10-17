extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write; // Trait to get flush()

fn main() {
    println!("Guess the secret number (between 0 and 100) !");

    let secret_number = rand::thread_rng().gen_range(0, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        let guess = get_input_number();
        // println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("It's bigger"),
            Ordering::Greater => println!("It's smaller"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/// Gets a number from standard input
fn get_input_number() -> u32 {
    loop {
        print!("Please input your guess : ");
        io::stdout().flush().expect("Exception while flushing standard output");
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => match guess.trim().parse() {
                Ok(parsed_value) => {
                    return parsed_value;
                }
                Err(e) => println!("\"{}\" is not a number : {}", guess.trim(), e)
            }
            Err(e) => println!("Error : {}", e)
        }
    }
}
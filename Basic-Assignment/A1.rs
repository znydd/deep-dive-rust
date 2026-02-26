use rand::prelude::*;
use std::io::stdin;

fn main() {
    let mut rng = rand::rng();
    let jackpot: u8 = rng.random_range(1..=50);
    let mut limit: u8 = 5;

    println!("Number Guessing Game");
    println!("Guess a number between 1-50");
    loop {
        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            println!("Failed to read the input try again!!");
            continue;
        }
        let valid_number: u8 = match input.trim().parse::<u8>() {
            Ok(num) if (1..=50).contains(&num) => num,
            _ => {
                println!("Invalid Input. Guess valid number between 1-50");
                continue;
            }
        };
        limit -= 1;
        if valid_number == jackpot {
            println!("Congratulations! You guessed the correct number.");
            break;
        } else if valid_number < jackpot {
            println!("Too low! Try again.");
        } else {
            println!("Too high! Try again.");
        }
        if limit == 0 {
            println!("You have reached the limit of guesses.");
            break;
        }
    }
}

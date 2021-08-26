use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;

const MAX_GUESSES: i32 = 10;

fn format_guesses(guesses: &Vec<i32>) -> String {
    guesses
        .into_iter()
        .map(|g| format!("{:02}", g))
        .collect::<Vec<String>>()
        .join(", ")
}

fn print_hint(low_guesses: &Vec<i32>, high_guesses: &Vec<i32>) {
    println!(
        "{} __ {}",
        format_guesses(low_guesses),
        format_guesses(high_guesses)
    );
}

fn parse_guess(guess: &str) -> Result<i32, ParseIntError> {
    guess.trim().parse::<i32>()
}

fn insert_sorted_guess(guesses: &mut Vec<i32>, new_guess: i32) -> &Vec<i32> {
    match guesses.binary_search(&new_guess) {
        Ok(_pos) => println!("You've already guessed {}.", new_guess),
        Err(pos) => guesses.insert(pos, new_guess),
    };

    guesses
}

fn main() {
    let mut rand_gen = rand::thread_rng();
    let mut guess_count: i32 = 0;
    let mut guess = String::new();

    let secret: i32 = rand_gen.gen_range(0..99);
    let mut low_guesses = Vec::new();
    let mut high_guesses = Vec::new();

    while guess_count < MAX_GUESSES {
        guess.clear();

        print!("Guess the secret > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut guess).expect("Failed to read line");

        match parse_guess(&guess) {
            Ok(guess) => {
                guess_count += 1;

                if guess == secret {
                    println!("You guessed it.");
                    return;
                }

                if guess > secret {
                    println!("{} is too high.", guess);
                    insert_sorted_guess(&mut high_guesses, guess);
                }

                if guess < secret {
                    println!("{} is too low.", guess);
                    insert_sorted_guess(&mut low_guesses, guess);
                }

                print_hint(&low_guesses, &high_guesses);
            }
            Err(_e) => println!("You must guess a number."),
        };
    }

    println!("You ran out of guesses. The secret was {}.", secret);
}

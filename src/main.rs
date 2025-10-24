use std::ops::RangeInclusive;

const MIN_VAL: u8 = 0;
const MAX_VAL: u8 = 20;
fn main() {

    println!("Your computer has challanged you to a guessing game!");
    println!("You will be in this game until you can read the computers mind!");
    println!("The computer will think of a random number between {} and {}.", MIN_VAL, MAX_VAL);
    println!("You will be stuck in this game until you have made the correct guess. Good luck!"); 

    let mut s = String::new();
    loop {
        s.clear();
        println!("Tell me your guess: ");
        if let Err(e) = std::io::stdin().read_line(&mut s) {
            println!("Could not read input: {}", e);
            continue;
        }
        if let Ok(user_guess) = s.trim().parse::<u8>() {
            if !RangeInclusive::contains(&(MIN_VAL..=MAX_VAL), &user_guess) {
                println!("Please select a value between {} and {}", MIN_VAL, MAX_VAL);
                continue;
            }
            let cmp_guess = rand::random_range(MIN_VAL..=MAX_VAL);
            if cmp_guess == user_guess {
                println!("You did it! You both guessed {}. You're free!", cmp_guess);
                break;
            }
            else if user_guess > cmp_guess {
                println!("Your number {} does not correspond the what the computer thought of. Your guess {} is higher than the computers number {}", user_guess, user_guess, cmp_guess);
                continue;
            }
            else {
                println!("Your number {} does not correspond the what the computer thought of. Your guess {} is lower than the computers number {}", user_guess, user_guess, cmp_guess);
                continue;
            }
        }
        else {
            println!("Please select a value between {} and {}", MIN_VAL, MAX_VAL);
            continue;
        }
    }
}

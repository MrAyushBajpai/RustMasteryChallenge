use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut counter: i32 = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number game!");

    loop{
        print!("Enter your guess: ");
        io::stdout().flush().expect("Unable to flush stdout()");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to get input");
        
        let guess: u32 = match guess.trim().parse(){
             Ok(num) => num,
             Err(_) => {
                println!("Enter a number!");
                continue;
             },
        };

        counter += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("\nYou Win! The secret number was {secret_number}");
                println!("You took {counter} guesses.\n");
                break;
                },
        }
    }
}

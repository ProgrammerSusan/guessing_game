
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 10;
    loop {
        println!("Please input your guess.");
        
        println!("You have {} guesses left.", guesses);

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
            
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small!".red());
                guesses -= 1;
                if guesses == 0 {
                    println!("{}", "You lose!".red());
                    break;
                }
            }
            Ordering::Greater =>  {
                println!("{}", "Too big!".red());
                guesses -= 1;
                if guesses == 0 {
                    println!("{}", "You lose!".red());
                    break;
                }
            }
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
    }
}
}
 
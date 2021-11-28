use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("Guessing Game version 0.1!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Guess : ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input");
                    continue;
                }
            };
        
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            },
        }
    }
}

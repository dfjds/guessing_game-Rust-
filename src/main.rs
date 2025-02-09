extern crate rand;  // imports external crate

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Guess a number between 1 and 100: ");
    let secret_num = rand::rng().random_range(1..101);  // Generaye random int within range
    let mut tries = 0;

    loop {  // infinite loop untill completion
        let mut input = String::new();  // creats mutable var as empty growable string
        io::stdin().read_line(&mut input).expect("Failed to read line");  // Reads line
        
        // Trims whitspace. Parses and converts to int. Expects non integer results
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > secret_num {
            println!("Lower");
            tries += 1;
        }
        else if guess < secret_num {
            println!("Higher");
            tries += 1;
        }
        else {
            println!("You got it!!");
            tries += 1;
            println!("It took you {} tries", tries);
            break;  // breaks the loop
        }
    } 
}

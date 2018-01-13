extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number (it's a secret)");

    loop {
        let mut guess = String::new();

        println!("Please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small :("),
            Ordering::Greater => println!("Too large :("),
            Ordering::Equal => {
                println!("Just right  :)");
                println!("You win!");
                break;
            }
        }

        println!("");
    }
}

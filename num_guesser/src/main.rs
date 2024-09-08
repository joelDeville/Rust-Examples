use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    println!("Guess the number!"); // macro, not a function

    let secret_num = rand::thread_rng().gen_range(1..=100); // range from 1-100, inclusive
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // mutable string variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

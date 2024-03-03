use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the nubmer!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Generated number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You have guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small!"),
            Ordering::Equal => {
                println!("You have guessed right!");
                break;
            }
            Ordering::Greater => println!("You have guessed too high!"),
        };
    }
}

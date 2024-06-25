use std::{cmp::Ordering, io::{self, Read}};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..10);

    println!("Guess a number");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Guess is invalid");

        let guessed_number: i32 = guess.trim().parse().expect("Invalid number");


        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Its smaller than the secret number"),
            Ordering::Greater => println!("Its greater than the secret number"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
    println!("\nGoodbye!\n")
}

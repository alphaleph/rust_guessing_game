use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Rust Guessing Game.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please enter a guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse()
        .expect("Please type an integer!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Try higher."),
        Ordering::Greater => println!("Try lower."),
        Ordering::Equal => println!("You got it!")
    }

}
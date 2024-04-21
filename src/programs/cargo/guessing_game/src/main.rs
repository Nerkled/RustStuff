use std::io;    // IO input library 
use rand::Rng;  // random number generator library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..= 100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");   

    let mut guess = String::new();          // variable to store the user input, also a mutable variable meaning it can change
                                            // String:: new() is a function that returns a new mutable instance of a string
    io::stdin()                             // type that represents a handle to the standard input for your terminal
        .read_line(&mut guess)              // read_line method on the standard input handle to get input from the user, & indicates that this agument is a refence
        .expect("Failed to read line");
    println!("You guessed: {guess}");       // {} placeholder for the value of the variable guess
}

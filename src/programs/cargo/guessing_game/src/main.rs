use std::io;    // IO input library 
use std::cmp::Ordering; // comparison library, enum that has less, greater, equal
use rand::Rng;  // random number generator library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..= 100);

    // println!("The secret number is: {secret_number}");



    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // undersocre acts as a catch all value
        };  // We changed this here because we want to handle the error better instead of a crash 
            // .trim()
            // .parse()
            // .expect("Please type a number!");
        

        // let mut guess = String::new();          // variable to store the user input, also a mutable variable meaning it can change
        //                                         // String:: new() is a function that returns a new mutable instance of a string
        // io::stdin()                             // type that represents a handle to the standard input for your terminal
        //     .read_line(&mut guess)              // read_line method on the standard input handle to get input from the user, & indicates that this agument is a refence
        //     .expect("Failed to read line");
        println!("You guessed: {guess}");       // {} placeholder for the value of the variable guess

        match guess.cmp(&secret_number) {       // Mathes are made up of arms, arms consist of a pattern to match against 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

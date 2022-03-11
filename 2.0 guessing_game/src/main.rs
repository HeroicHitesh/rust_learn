use std::io;    // Import the io module for I/O operations
use rand::Rng;  // Import the Rng module for random number generation
use std::cmp::Ordering;  // Import the Ordering module for comparing numbers

fn main() {
    println!("Guess the number!");

    // Generating a Secret Number
    let secret_number = rand::thread_rng().gen_range(1..101);    // Generate a random number between 1 and 100

    println!("The secret number is: {}", secret_number);

    // Allowing Multiple Guesses with Looping
    loop {
        // Processing a Guess
        println!("Please input your guess.");

        // Variables are immutable by default, use mut to create mutable variables
        let mut guess = String::new();    // Create a mutable variable bound to new empty instance of a String

        io::stdin()    // Get the standard input stream
            .read_line(&mut guess)    // Read a line from the standard input stream and append it to the guess variable, & indicates reference
            .expect("Failed to read line");    // If the read_line method returns an error, print the error message and exit the program
        
        // Handling Invalid Input
        // let guess: u32 = guess.trim().parse()    // Shadowing guess variable to convert it to an unsigned 32-bit integer
        //     .expect("Please type a number!");    // If the guess variable is not a number, print the error message and exit the program
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);    // Use placeholders to print the guess variable

        // Comparing the Guess to the Secret Number
        match guess.cmp(&secret_number) {    // Compare the guess variable to the secret_number variable
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Quitting After a Correct Guess
                println!("You win!");
                break;
            }
        }
    }
}

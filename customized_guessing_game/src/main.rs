use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generating a Secret Alphabet
    let secret_alphabet = rand::thread_rng().gen_range(b'a'..b'z') as char; // Generate a random lower case alphabet

    println!("The secret alphabet is: {}", secret_alphabet);

    loop {
        // Processing a Guess
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Handling Invalid Input
        let guess: char = match guess.trim().parse() {
            Ok(alphabet) => alphabet,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Comparing the Guess to the Secret Alphabet
        match guess.cmp(&secret_alphabet) {
            Ordering::Less => println!("Letter is too small!"),
            Ordering::Greater => println!("Letter is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

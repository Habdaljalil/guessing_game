use std::io; // I think that this is a library
             // EDIT: Yes, the io library is coming from the standard(std) library
use rand::Rng;
// Imports the Rng library from rand
// EDIT: Not exactly, this activates a trait called Rng from the rand library

use std::cmp::Ordering;
// I think Ordering is a trait of cmp, which could be a sub-library

fn main() {
    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // OK, so this calls rand to make a random number generator from the range 1-100; is it doing this on a single thread?
    // Why is the syntax for ranges like this?
    // EDIT: Yes, the random number generator is nade from the current thread of execution
    // EDIT: gen_range method is called on the new number generator; this comes from the Rng trait
    // EDIT: the syntax is start..=end (inclusive values)
    loop {
        // println!("The secret number is: {secret_number}");
        println!("Please input your guess.");

        let mut guess = String::new();
        // I think let means --> new value
        // I think mut means mutable --> this is NOT constant; it's a variable
        // guess is just the name
        // Is "String" a class? It looks like it because of the ::new()
        // EDIT: Variables are IMMUTABLE by DEFAULT; mut is applied to let them be changed
        // EDIT: String::new() --> function that makes new instance of String

        io::stdin() // Are you calling the library?
            .read_line(&mut guess) // This is reading the terminal line. I think the ambersand is used for writing the value of the pointer to guess
            .expect("Failed to read line"); // I think that this is for exceptions.
                                            // EDIT: The .readline --> calls this method --> stores the result in the guess variable; & = reference; use &mut to make reference writable
                                            // EDIT: .readline returns an enum called Result; .expect is the case in which the enum's value, called a variant, is Err(not OK)

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // the _ is a catch-all value; we want to match ALL errors
            // The "continue" calls another loop, preventing the rest of the current loop's execution
        };
        // Does Rust handle type conversion automatically?
        // EDIT: Trim removes additional characters and parse converts the string to another type
        // EDIT: The : u32 lets Rust know that I'm annotating the type(unsigned, 32-bit integer)

        println!("You guessed: {guess}"); // Is this formatting?
                                          // EDIT: The curly braces define a placeholder

        match guess.cmp(&secret_number) {
            // I think we need to convert &secret_number to a string
            // EDIT: This is because guess is &[string]
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // ends the program's loop after the player successfully guesses
            }
            // Cmp method compares two valuesl; can be called on anything

            // EDIT: match exp. is made of "arms" --> consist of patterns to match against
            // The match takes the output of the guess, such as Ordering::Less --> looks up the "arm", then executes the pattern
        }
    }
}

use std::io; // i/o from standard library
// if something isn't in the prelude, must use `use` to bring into scope

fn main() {
    println!("Guess the number!");
    println!("Input your guess.");
    let mut guess = String::new();
    // let creates a variable, mut = mutable. to make immutable, don't add mut
    // let apples = 5 ==> immutable
    // String::new() = new instance of string
    // - :: = new is a fn associated with the String type
    // currently empty
    io::stdin() // use io library, read from stdin
        .read_line(&mut guess) // take user's input and append to string without overwriting contents
        // must be mutable so that method can change the string and actually append the input
        // & = reference
        .expect("Failed to read line");
        // read_line returns a Result value of enum type - Ok | Err
        // instance of Result has a .expect() method to call
        // if Ok is returned, then .expect() returns the received value
    println!("You guessed: {guess}")
    // can also do this:
    // println!("You guessed: {}", guess) - this is preferable when computing expressions
}

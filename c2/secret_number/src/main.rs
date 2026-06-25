use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // infinite loop
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100); // 1 - 100 inclusive
        // thread_rng = an rng that is local to the current execution thread, seeded by OS
        // gen_range = start..=end, inclusive upper + lower bounds
        // default number = i32
        println!("Input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // unsigned = u32
        // following line utilizes shadowing, allows for reuse because of diff data types
        // useful for converting a value from one type to another
        // use match + arm to ignore non-number inputs
        // parse returns a Result enum, so we can check if it equals Ok or Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // return the num value that parse produces, into guess
            Err(_) => continue, // continue the loop if any other non-num is passed in
            // underscore is similar to a catch-all
        };
        // trim = eliminate whitespace at beginning or end
        // parse = convert string to a number
        // the `: u32` part tells Rust to convert to u32

        println!("You guessed: {guess}");
        println!("The secret number is {secret_number}");

        // match is made up of "arms"
        // each arm is a pattern to "match" against, and runs code
        // similar to switch statements
        match guess.cmp(&secret_number) { // compare guess to secret_number
            Ordering::Less => println!("Too small"), // Another enum, three values
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break; // kills program when you win
            }
        }
    }
}

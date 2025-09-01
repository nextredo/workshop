// Rust's *prelude*
    // Automatically brought into scope here
    // This is a set of things accessible without any imports by default
    // https://doc.rust-lang.org/std/prelude/index.html

// Bring things into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// `fn` indicates function declaration
fn main() {
    // Standard prints, see chapter 1
    println!("Guess the number!");

    // Get thread-local, OS-seeded RNG
    // `.gen_range()` part of Rng trait - in-scope because of the `use rand::Rng`
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Secret no is {secret:?}");

    loop {
        println!("Please input your guess.");

        // Define new variable
            // Mutation
                // Vars are immutable by default
                // Make it mutable with the `mut` keyword
            // `=`
                // Bind var to something
            // String
                // Growable, UTF-8 encoded text
                // Heap-allocated
            // `::`
                // Notes that the `new` function is associated with the `String` type
            // `new`
                // Common name for functions that make a new value
        let mut guess = String::new();

        // Broken up across a few lines for readability
            // Newlines on new methods by convention
        // `io::stdin()`
            // Can use through `std::io::stdin()` without the `use std::io`
            // Returns a `std::io::Stdin` instance
        // `.read_line()`
            // Appends stdin to a string
            // `&mut guess`
                // `&` indicates this is arg is a *reference*
                // References are immutable by default
                // Make it mutable with the `mut` keyword
                // Same behaviour as vars ^^
        // `.expect()`
            // `.read_line()` returns a `Result`
                // `Result` is an enumeration/enum
                // Each value it can take is called a variant
                // Can be `Ok` or `Err`
                // `Ok` contains the value (if it succeeded)
                // `Err` contains error info (if it failed)
            // Will return the value of the result if `Result` is `Ok`
            // Returns number of bytes in the input in our case
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");

        // Variable shadowing (2nd thing named "guess")
        // Often used when converting between types
        // More in chapter 3
        // `.trim()`
            // Removes leading/trailing whitespace
        // `.parse()`
            // Uses the u32 type annotation to parse the string into that type
            // Returns `Result`, so it can signal failure
        // Rust will infer the type of `secret` to be `u32` like this var
            // Since they get compared together later
        let guess: u32 = guess.trim()
            .parse()
            .expect("Please type a number");

        // Macro, `{}` is a placeholder
        // Can put var name directly in them
        // Or do comma stuff after the string (w/ empty `{}`s)
        println!("You guessed: {guess}");

        // `match` statement consists of *arms*
        // Arms consist of a *pattern* and the code should run if that pattern matches
        // Rust looks thru arms in turn
        // `match` covered further in chapters 6, 19
        // Useful for ensuring your code covers all possible paths
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

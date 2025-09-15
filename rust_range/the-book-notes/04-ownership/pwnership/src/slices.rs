// Function that takes a string reference
// Returns the first "word" in the string
  // i.e. the first set of characters before a space
// Ignoring the possiblity of multibyte UTF-8 characters here
fn first_word_bad(s: &String) -> usize {
    // Naive (sliceless) implementation

    // Convert string to byte array
    let bytes = s.as_bytes();

    // `iter()`
        // Create iterator
        // More in chapter 13
    // `enumerate()`
        // Wraps result of `iter()` in a tuple
        // (index, item)
        // More in chapter 6
    for (i, &item) in bytes.iter().enumerate() {

        // Byte literal
        if item == b' ' {
            return i;
        }
    }

    s.len()

    // Returns end of word index
    // The problem with this (as a separate value) is that it's decoupled
    // from the `String` object.
    // If the string changes, the variable used to store the result
    // from this function will remain unchanged.
    //
    // Values calculated from data that isn't in sync with it
}


// **`str` is the ultra primitive string type**
// Like `const char*` in C I guess?
// This is the type of string slices, and the type of string literals
// It's just a pointer and a length under the hood
fn first_word(s: &String) -> &str {
    // How can we return something talking about *part* of a string?
    // Enter, slices

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


#[allow(unused_variables)]
pub fn main() {
    let mut s = String::from("aaa bbb ccc");
    let len = s.len();
    let word = first_word_bad(&s);

    // `word` can get out of sync!!
    dbg!(&word);
    s.clear();
    dbg!(&word);

    // Slices :)
    // Created using a `Range` within square brackets
    // Internally, this is a pointer and a length
    dbg!(&s);

    // [0, 4)
    dbg!(&s[0..4]);
    dbg!(&s[..4]);

    // [3, end]
    dbg!(&s[3..len]);
    dbg!(&s[3..]);

    // [start, end]
    dbg!(&s[0..len]);
    dbg!(&s[..]);

    // Function that returns a string slice
    // Since it now returns a kind of reference (slice), Rust's borrow rules come into place
    // It returns an immutable reference, so we can't make a mutable one while that's alive
    let word = first_word(&s);
    // `word` not used after this point, it's lifetime ends here

    s.clear();

    // Can't add, would prolong immutable reference lifetime
    // so it overlaps w/ the mutable reference's livetime
    // dbg!(&word);
}


// TODO listing 4-8 should have a Ferris "logic error" icon? Maybe?

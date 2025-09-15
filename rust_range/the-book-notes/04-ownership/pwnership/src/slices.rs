// Function that takes a string reference
// Returns the first "word" in the string
  // i.e. the first set of characters before a space
// Ignoring the possiblity of multibyte UTF-8 characters here
fn first_word(s: &String) -> usize {
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
    // Values calculated from data that aren't in sync with it
}

// How can we return something talking about *part* of a string?
// Enter, slices


pub fn main() {
    let s = String::from("aaa bbb ccc");
    first_word(&s);
}

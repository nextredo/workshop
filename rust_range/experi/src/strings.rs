pub fn main() {
    let s = String::from("abcdef 🧙‍♂️");
    let len = s.len();

    dbg!(&s);
    dbg!(&len);

    // Attempting to index into the middle of a UTF-8 multibyte char
    dbg!(&s[4..6]); // ok
    dbg!(&s[5..7]); // ok
    dbg!(&s[6..8]); // yikes
}

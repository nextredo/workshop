pub fn main() {
    // Raw string shenanigans --------------------------------------------------
    let raw_s = r#" Raw string \\ "" "#;
    dbg!(raw_s);

    // Other string stuff ------------------------------------------------------
    let s = String::from("abcdef 🧙‍♂️");
    let len = s.len();

    dbg!(&s);
    dbg!(&len);

    // Attempting to index into the middle of a UTF-8 multibyte char
    dbg!(&s[4..6]); // ok
    dbg!(&s[5..7]); // ok
    dbg!(&s[6..8]); // yikes

    // Char-indexing
    dbg!(&s.chars().nth(2));
}

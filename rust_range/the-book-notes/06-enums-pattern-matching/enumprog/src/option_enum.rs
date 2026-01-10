// The standard library option enum looks like this
// So commonly used, it's part of the Rust prelude
    // Variants None and Some are too
// <T> is a generic type (like C++ template I guess)
enum MyOption<T> {
    None,
    Some(T),
}

pub fn main() {
    // Type inference
    let num  = Some(5);
    let char = Some('a');

    // Type annotation
    let absent: Option<i32> = None;

    // Can't use option types directly
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Can't add directly
    // These are different types
    // And unlike C/C++, type coercion is not allowed here
    //
    // Compiler ensures values like `i8` are always valid
    // So we never have to check that for null
    // Whereas use of `Option<>` means the compiler forces us
    // to check it's not absent first
    //
    // Can safely assume values without an `Option<T>` are always valid
    // This is a deliberate design feature of Rust
    // (to limit null's pervasiveness)
    //
    // let sum = x + y;

    // Must do something like this instead
    // WARN: This will panic if `y` is `None`
    let sum = x + y.unwrap();

    // However, it's better to handle all variants explicitly
    // Using a `match` or `if let` construct
}

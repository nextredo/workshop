#![allow(dead_code)]

mod better_rects;

// Explicit opt-in to the `Debug` automatically generated trait implementation
// Outer attribute
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ------------------------------
// 3 ways of writing this program

// With free variables
// Not good, unclear that `width` and `height` vars are related
fn area_free(width: u32, height: u32) -> u32 {
    width * height
}

// With (unnamed) tuples
// Vars are now related, but it's unclear what they are
// Doesn't matter in this example (multiplication is commutative)
// However, it might in others (e.g. if dimensions were for a trapezoid)
fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// With structs
// Single, grouped parameter
// Clearer about what's what
// Easier to borrow rather than transfer ownership (move)
// Can add functionality with derived traits
fn area_struc(rect: &Rectangle) -> u32 {
    // Accessing fields of borrowed struct is fine (doesn't move it)
    rect.width * rect.height
}

fn main() {
    let rec = Rectangle {
        width: 5,
        height: 2
    };

    // Error - `Rectangle` doesn't implement std::fmt::Display
    // Compiler warns about this, suggests using `:?`
    // Using `:?` doesn't help though - needs to implement `Debug`
    // Compiler states you can use `#[derive(Debug)]`
    println!("rectangle = {rec:?}");

    // You can print out slightly better formatted/styled debug info like so
    println!("rectangle = {rec:#?}");

    // This macro takes ownership
    // Prints to stderr (more in chapter 12)
    // Returns ownership though
    dbg!(area_struc(&rec));

    // Can use `dbg!` like so:
    let rec = Rectangle {
        width: dbg!(6 * 4),
        height: 2
    };

    // Not taking ownership here
    dbg!(&rec);

    better_rects::main();
}

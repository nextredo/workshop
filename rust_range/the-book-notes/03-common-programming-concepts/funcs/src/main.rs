fn other_fn() {
    dbg!();
}

// Function with *parameters*
// Values provided to the parameters are called *arguments*
// These two terms are often used interchangably though
// ----
// Must declare types in func signatures
  // Needs type annotations
  // Means compiler can do type inference elsewhere often
fn params(x: i32, y: f64) {
    dbg!(x);
    dbg!(y);
}


fn main() {
    // Doesn't have to be defined before it's used
    // Just has to be in accessible in the caller's scope
    other_fn();
    params(5, 6.6);
}

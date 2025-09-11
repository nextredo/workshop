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


fn statements() {
    // Things that are statements:

    // Function definitions (like this one)

    // Doesn't return a value
    // Can't do this;
        // let x = (let y = 4);
    // This is *unlike* languages like C & Ruby
        // Where assignments return the value of the assignment
    let _y = 6;

    // Statements *have ending semicolons*
}


fn expressions() {
    // Things that are expressions:

    // Function calls
    other_fn();

    // Math
    let _ = 3 + 3;

    // Macro calls
    println!("hi mum");

    // Block scopes
    { }

    // Expressions *do not include ending semicolons*
}


fn returners(early_ret: bool) -> i32 {

    // Returns are unnamed
    // Require declaration in the function signature
    // Synonymous w/ the final expression in a function's body block
        // Last value returned implicitly

    // Can return early with the `return` keyword
    if early_ret {
        return 3 + 3;
    }

    // Don't use a return here by convention
    6 + 4
}


fn main() {
    // Doesn't have to be defined before it's used
    // Just has to be in accessible in the caller's scope
    other_fn();
    params(5, 6.6);

    statements();
    expressions();
    returners(false);
}

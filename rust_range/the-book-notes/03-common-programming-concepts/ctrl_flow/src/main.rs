fn ifs() {
    let num = 6;

    // Keyword followed by a condition
    if num > 3 {
        // This is one *arm* of the `if` statement (like arms in a `match`)
        println!("Condition evaluated true");
    } else {
        println!("Condition evaluated false");
    }

    // The condition in an `if` *must* be a `bool`
    // Rust will not automatically attempt to convert types into booleans
    // if 3 {
    //     println!("hello")
    // }

    // Checks each `if` expression in turn
    // Executes the first to evaluate to `true`
    // Doesn't check the rest after it hits one that's `true`
    // Use `match` instead for large conditionals (powerful Rust branching construct)
    if num % 3 == 0 {
        println!("divisible by 3");
    } else if num % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("divisible by neither 2 or 3");
    }

    // If is an expression, we can use it in `let` bindings
    // All arms of the `if` must return the same type
    let cond = true;
    let num = if cond { 5 } else { 6 };
    dbg!(num);
}


fn loops() {

}


fn main() {
    ifs();
    loops();
}

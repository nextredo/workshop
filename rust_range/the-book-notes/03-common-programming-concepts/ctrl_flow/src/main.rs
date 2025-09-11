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
    // Execute code forever or until told to stop
    loop {
        println!("hehe");

        // Supports the following keywords

        // Skip over remaining code in the loop
        // continue;

        // Exit the loop
        break;
    }


    // Loops are expressions and can return values
    // Good for checking if a thread did its job or something
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };
    dbg!(result);


    // Loops can have labels
    // These can be used to break or continue in nested loops
    let mut num = 0;

    'outer: loop {
        dbg!(num);

        loop {
            num += 1;
            println!("incremented to {num}");

            if num % 10 == 0 {
                // Exits the outer loop
                // Could also continue the outer loop
                // Can return values from these too
                break 'outer;
            }
        };
    };
}


fn whiles() {
    // Hides the break from the user
    // Bit nicer than manually adding the condition to the loop
    let mut no = 3;
    while no > 0 {
        println!("{no}!");
        no -= 1;
    }
    println!("LIFTOFF!!");

    // Error prone indexing approach -----------------------------
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // What if you forget to update the condition?
    // Or the index variable?
    // Possible to skip values, attempt invalid ones etc.
    // Slow too - compiler adds code to check index is in-bounds on each iteration
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}


fn fors() {
    // Much better than using a `while` to loop thru a collection
    // Most commonly used loop construct in Rust
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }


    // Rewritten "LIFTOFF!!" loop, with `for`
    // Uses a `Range` provided by the std library
    // `.rev()` reverses the range
    for no in (1..4).rev() {
        println!("{no}!");
    }
    println!("LIFTOFF!!");
}


fn main() {
    ifs();
    loops();
    whiles();
    fors();
}

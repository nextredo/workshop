fn vars() {
    let mut x = 5;
    const CONSTANT: u64 = 1 + 1;

    dbg!(x);
    x = 6;
    dbg!(x);

    dbg!(CONSTANT);
}

fn tuples() {
    // Example tuple definition
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Pattern matching to destructure a tuple
    let (x, y, z) = tup;
    println!("Destructured tuple is: {x}, {y}, {z}");

    // Access by period indexing
    // (1st index is 0)
    let elem = tup.0;
    println!("Element at position is {elem}");

    // Special tuple
    // Called *unit*
    // Represents an empty value / empty return type
    // If expressions don't return anything, they implicitly return the unit value
    let unit_type = ();
    dbg!(unit_type);
}


fn arrays() {

}


fn shadows() {
    // 1st `x`
    // *shadowed* by 2nd `x`
    let x = 5;

    // 2nd `x`
    // *overshadows* 1st `x`
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner scope x: {x}");
    }

    println!("Outer (end of) scope x: {x}");

    // Shadowing w/ different types allowed
    let _spaces = "   ";
    let _spaces = _spaces.len();
}


fn main() {
    vars();
    shadows();
    tuples();
}


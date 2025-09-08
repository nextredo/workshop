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
    // Deduced (inferred) array length
    let a = [1, 2, 3, 4, 5];

    // Array type annotation
    let b: [i32; 5] = [6, 7, 8, 9, 10];

    // Specify all the same elements in an array
    // This creates an array of 5 elements, all initialised to 3
    let c = [3; 5];

    dbg!(a);
    dbg!(b);
    dbg!(c);

    println!("0th value: {}", a[0]);

    // This panics!
    // *runtime error*
    // index cannot be >= array length
    // more about handling errors well in chapter 9
    println!("n+1th value: {}", a[a.len() + 1]);
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
    arrays();
}


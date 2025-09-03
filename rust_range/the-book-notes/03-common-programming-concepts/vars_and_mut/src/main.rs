fn vars() {
    let mut x = 5;
    const CONSTANT: u64 = 1 + 1;

    dbg!(x);
    x = 6;
    dbg!(x);

    dbg!(CONSTANT);
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
}


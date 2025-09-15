mod references;

fn dropping() {
    // `::` operator lets us namespace the function (more in chapter 5)
    let mut s = String::from("hello"); // s valid here onwards
    s.push_str(" yeet");

    dbg!(&s);

    // end of scope calls `drop` on `s`
}


fn moving() {
    // Bind `5` to `x`
    let x = 5;

    // Copy value in `x`, bind it to `y`
    // This happens, since integers are simple, fixed-size stack-based items
    // No reason to invalidate `x` when `y` is created (unlike 2x free for `String`s)
    // **No difference between a deep and shallow copy here**
    let y = x;

    dbg!(&x);
    dbg!(&y);

    // ----------------
    // String version
    // A string is really:
        // On the stack
            // Pointer to data
            // Data length (number of bytes currently in use)
            // Capacity (number of bytes the allocator gave us)
        // On the heap
            // The data (pointed to)
    let s1 = String::from("yeetus");

    // Copies the *stack* part of `s1`
    // Does *not* copy the heap part of it
    // Analogus to a *shallow copy*
    // However, it's actually a *move*
        // Since `s1` is invalidated by this operation
    let s2 = s1;


    // If s1 and s2 both called `drop` when the scope ended, we'd have a double-free condition
    // Hence why it's moved instead

    //dbg!(&s1);
    dbg!(&s2);

    // -----------------
    // Assigning over an existing variable
    let mut s = String::from("hi (x3)");

    // When you do this, Rust calls `drop` on the original value immediately
    s = String::from("ahoy");

    dbg!(&s);
}


fn clone_wars() {
    let s1 = String::from("hi");

    // Method syntax discussed in chapter 5
    // `.clone()` will "deep copy" this object
    // Shows directly that something non-trivial is going on
    let s2 = s1.clone();

    dbg!(&s1);
    dbg!(&s2);
}


fn own_it(words: String) {
    dbg!(words);
    // String goes out of scope, `drop` called
}


fn borrow_it(value: i32) {
    dbg!(value);
    // Copied int goes out of scope, nothing special happens
}


fn pwn_fns() {
    // Kinda like pass by value vs. pass by reference??

    let s = String::from("hi");
    own_it(s);                  // `s` moved into the function
    //dbg!(s);                  // Can't do it

    let x = 3;
    borrow_it(x);               // `i32` implements `Copy`, so that trait is used
    dbg!(x);                    // So it can be used after (doesn't move)
}


fn gives_ownership() -> String {
    let var = String::from("have it bro");
    var
}


fn takes_then_gives_back(value_pls: String) -> String {
    let len = value_pls.len();
    println!("len of {len}");
    value_pls
}


fn returners() {
    let s1 = gives_ownership();

    // Tranferring ownership into and out of everything all the time
    // would be a giant pain in the ass
    // Thankfully, we have *references*
    let s2 = String::from("hi");
    let s3 = takes_then_gives_back(s2);

    dbg!(&s1);
    //dbg!(&s2);
    dbg!(&s3);

    // s1 goes out of scope (dropped)
    // s2 was moved, so nothing happens
    // s3 goes out of scope (dropped)
}


fn main() {
    dropping();
    moving();
    clone_wars();
    pwn_fns();
    returners();

    references::main();
}

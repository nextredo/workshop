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
    let y = x;

    // This happens, since integers are simple, fixed-size stack-based items
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

}


fn main() {
    dropping();
    moving();
    clone_wars();
}

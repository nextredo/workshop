// Issue with this is that it must pass ownership around
// Meaning, we have to pass the string into this func, and back out
fn calc_len_own(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}


// Use a reference instead
// Dereferencing achieved with `*` - chapter 8 & 15 stuff
// Type of `s` is a *reference to a `String`*
fn calc_len_ref(s: &String) -> usize {
    s.len()

    // `s` goes out of scope
    // Not dropped, since it's not owned
    // Don't need to return ownership, we never had it
}


// `mut` in function signature makes it apparent that the value may change
fn change(s: &mut String) {
    s.push_str(" more stuff");
}


fn multi_mutable_borrow() {
    let mut s = String::from("hehe");

    {
        // Cannot create multiple mutable references at once
        let r1 = &mut s;
        // let r2 = &mut s;
        dbg!(r1);
    }

    // Can get around it with scoping (no simultaneous multiple mutable refs tho)
    {
        let r1 = &mut s;
        dbg!(r1);

        // r1 out of scope here
    }
    let r2 = &mut s;
    dbg!(r2);
}


fn multi_immutable_borrow() {
    #![allow(unused_mut)]

    let mut s = String::from("hello");

    {
        // Cannot have both immutable and mutable references simultaneously
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        // let r3 = &mut s; // BIG PROBLEM

        println!("{r1}, {r2}");
    }

    let r1 = &s; // no worries
    let r2 = &s; // no worries
    println!("{r1}, {r2}");
    // `r1` and `r2` not used after here
    // Therefore, their scopes end here

    let r3 = &mut s; // no worries
    println!("{r3}");
}


// The concept of *lifetimes* prevents this
// fn dangling_ref() -> &String {
//     let s = String::from("megahehe");
//
//     &s // We return a reference to `s`
// }      // `s` dropped here (out of scope)
// Would alternatively work fine if we moved ownership outta here


pub fn main() {
    let mut s1 = String::from("hi");

    // Passing ownership in
    // let (s2, len) = calc_len_own(s1);
    // dbg!(len);

    // Creating a reference is *borrowing*
    let len = calc_len_ref(&s1);
    dbg!(len);

    // Mutable reference
    change(&mut s1);

    multi_mutable_borrow();
    multi_immutable_borrow();
}

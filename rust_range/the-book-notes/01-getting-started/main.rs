// main func always runs first in rust programs
fn main() {
    // `{}` required around all function bodies
        // opening `{` on same line as func declaration
            // doesn't follow the K&R C/C++ Style
            // https://en.wikipedia.org/wiki/Indentation_style#K&R
        // format w/ `rustfmt`

    // macro (as indicated by `!`)
        // discussed in chapter 20
    // `;` indicates expression is over
    println!("Hello, world!");
}

// ------------------------------------------------------------
// step 1: compile with;
    // rustc main.rs
    // produces an output binary
    // also produces a `.pdb` for debug info on Windows
// step 2: run with;
    // ./main

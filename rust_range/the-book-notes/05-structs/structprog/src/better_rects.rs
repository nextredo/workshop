#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}


// Start an `impl` (implementation) block for `Rect`
// Everything in here is associated with `Rect`
impl Rect {

    // Using `&self` instead of `rec: &Rect`
    // Within an `impl` block, `&self` is short for `self: &Self`
        // Where `Self` is an alias for the type the impl block is for
    fn area(&self) -> u32 {
        // This immutably borrows self instead of taking ownership of it
        // We just want to read the data, not manipulate it
        // Can take ownership, borrow immutably or borrow mutably like any param
        self.width * self.height
    }

    // Can double up on method and field names
    // `instance.width` will refer to the field
    // `instance.width()` will refer to the method
    // This is excellent for "getters"
        // Basically, for read-only access to fields through public methods
        // Information hiding covered in chapter 7
    fn width(&self) -> bool {
        self.width > 0
    }
}


pub fn main() {
    let rec = Rect {
        width: 10,
        height: 20
    };

    // Call using "method syntax"
    dbg!(rec.area());
}

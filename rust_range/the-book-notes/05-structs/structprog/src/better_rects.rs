#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}


// Start an `impl` (implementation) block for `Rect`
// Everything in here is associated with `Rect`
impl Rect {
    // Using `&self` instead of `rec: &Rect`
    // *Within an `impl` block, `&self` is short for `self: &Self`*
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

    // Determines if another rectangle can fit entirely within this one
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    // Using the `Self` type alias
    // But doesn't require an instance of "self" to operate on
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


pub fn main() {
    let rec = Rect {
        width: 10,
        height: 20
    };

    let rec2 = Rect {
        width: 5,
        height: 15
    };

    // Call using "method syntax"
    dbg!(rec.area());

    // Check against another rect
    dbg!(rec.can_hold(&rec2));

    // Using associated functions
    // Namespaced by the struct
    // `::` used for associated functions and module namespaces (Chapter 7)
    dbg!(Rect::square(3));
}

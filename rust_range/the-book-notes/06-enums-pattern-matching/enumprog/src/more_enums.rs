// An example enum from the docs
// This is better than defining structs for each different kind of message
// As doing so makes a bunch of different types,
// and using them in functions becomes harder
enum Message {
    Quit,                       // No associated data
    Move { x: i32, y: i32 },    // Names fields, like a struct
    Write(String),              // Single piece of data
    ChangeColor(i32, i32, i32), // Tuple
}

impl Message {
    fn call(&self) {
        // Do something
    }
}

pub fn main() {
    let m = Message::ChangeColor(2, 3, 5);
    m.call();
}

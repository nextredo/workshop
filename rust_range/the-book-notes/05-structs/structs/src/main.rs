// Define a struct like so
// Name should describe the significance of the data grouped together
#[derive(Debug)]
struct User {
    // Each of these are *fields*
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn basic_instantiation() {
    // Create a struct *instance* (instantiate it)
    // Either the whole thing is mutable, or it's immutable
    let mut user = User {
        // key: value pairs
        // can be in any order (doesn't need to match definition)
        active: true,
        username: String::from("bilbo_swaggins"),
        email: String::from("notinmyshire22@gmail.com"),
        sign_in_count: 48,
    };
    dbg!(&user);

    // Only works if the instance is mutable
    user.email = String::from("whatinmyshire23@gmail.com");
}


fn main() {
    basic_instantiation();

    let user = User {
        // key: value pairs
        // can be in any order (doesn't need to match definition)
        active: true,
        username: String::from("bilbo_swaggins"),
        email: String::from("notinmyshire22@gmail.com"),
        sign_in_count: 48,
    };
    dbg!(&user);

    // Tuple, for comparison
    let hehe: (i32, i64) = (9, 20);
    dbg!(&hehe);
}

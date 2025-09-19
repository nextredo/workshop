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


// Rust also has *tuple structs*
// Like structs, do have a struct name
// Like tuples, don't have field names
// Good for
  // Giving whole tuple a name
  // Making tuple different from other tuples
  // When naming each field is verbose / redundant
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// We also have *unit-like structs*"
// (structs with no fields)
// Behaves like `()`
// Good for when you need to implement a trait on a type,
// but you don't ned to store data in the type itself
// More in chapter 10
struct AlwaysEqual;


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


fn field_init_shorthand(email: String, username: String) -> User {
    User {
        // Field init shorthand
        // Parameter names are the same as the fields
        // Rust uses (for example) username: username here
        // So it would work if we had a local variable called `username` instead
        active: true,
        username,
        email,
        sign_in_count: 48,
    }
}


fn struct_update_syntax(user: User) {
    // Without the update syntax
    let new_user = User {
        active: user.active,
        username: user.username,
        email: String::from("whaa"),
        sign_in_count: user.sign_in_count,
    };

    // With the update syntax
    // Specifies fields not explicitly set should have
    // the same value as the given instance
    // `..new_user` must come last
    let new_user_2 = User {
        email: String::from("whaa"),
        ..new_user
    };
    dbg!(new_user_2);

    // This moves values from `new_user` to `new_user_2` where applicable
    // We specified the email, so that's not moved
    // The primitive types implement `Copy` since they're stack-based, so that's ok
    // However, `username` has been moved into this new object
    //dbg!(new_user);
}


fn tuple_structs() {
    // These are different types
    // (since they're instances of different types)
    let _black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Destructuring
    let Point(_x, _y, _z) = origin;

    // Individual values
    println!("0: {}", origin.0);
}


fn main() {
    // Tuple, for comparison
    let hehe: (i32, i64) = (9, 20);
    dbg!(hehe);

    basic_instantiation();

    let user = field_init_shorthand(
        String::from("huh"),
        String::from("uhh")
    );
    dbg!(&user);

    struct_update_syntax(user);

    tuple_structs();
}

# Structs
- Group multiple related values
- It's the data attributes / members of an object
- Different from tuples
- Learn to associate functions with them
  - *Methods*

## Defining & Instantiating
- Like tuples - can hold multiple different types
- Unlike tuples - data is named
  - Access via names
  - Don't need to rely on ordering

## Ownership of Struct Data
- Chose `String` not `&str`
- This is a deliberate choice
- We want the struct to own all its data
  - and for that data to be valid as long as the struct is
- Structs can store references to data owned by something else
  - Through *lifetimes* (chapter 10)
  - Can't store data in structs without specifying lifetimes

```rust
struct User2 {
    active: bool,
    username: &'static str, // Lifetime specification
    email: &'static str,    // Lifetime specification
    sign_in_count: u64,
}
```

## Adding Useful Functionality with Derived Traits

## Method Syntax
- Functions (declare with `fn`)
  - Have parameters, return values
- Defined within the context of a struct (or enum/trait object, chapters 6, 18)
- First parameter is always `self`

### Reasons to use them
- Shorter syntax (`&self` usage instead of repeating the type everywhere)
- Organisational purposes
  - Groups related code together
  -

### Quirks
- Can double up on method and member names

### `->` operator
- C and C++ use `.` for methods/fields on objects directly, `->` for use indirectly (via pointers)
- Rust does not do this, no `->` equivalent
- Rust does *automatic referencing and dereferencing*
  - Methods are one of the few places this is used
- Works since methods have a clear receiver (`self`)
- Rust can definitively figure out if `self` is `&self`, `&mut self` or `self`
  - **Reading, mutating or consuming**

```rust
// Rust automatically adds `&`, `&mut`, `*`
// So that `p1` (the object) matches the method signature
p1.distance(&p2);

// So under the hood, it's really
(&p1).distance(&p2);

// Assume the distance looks like this
impl Point {
    // Immutable self borrow, immutable other point borrow
    fn distance(&self, point: &Point) -> u32 {
        // < distance calc code goes here >
    }
}
```

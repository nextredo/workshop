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

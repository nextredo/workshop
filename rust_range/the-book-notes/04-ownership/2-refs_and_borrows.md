# References
## Basic Def
- Like a pointer
- Guaranteed to point to a valid value, for the life of the reference

## Vs. C++
- The syntax of this is different to references in C++

```cpp
#include <iostream>

// Takes an int reference
int func(int& thing) {
    std::cout << thing << "\n";
}

int main() {
    int hi = 5;

    // Pass object directly
    // will be taken as a reference (silently)
    func(hi);
    return 0;
}
```

## Mutable References
### Rule 1 - Mutable references
- **Can't have multiple mutable references to the same data at once**
  - This allows Rust to prevent data races at compile time
  - Data races happen when
    - >=2 pointers accessing the same data at once
    - At least one of the pointers is used to write to the data
    - No mechanism to synchronise data access

### Rule 2 - Simutaneous mutable & immutable references
- **Can't have mutable and immutable references to the same data at once**
  - Users of immutable references don't expect their data to change during usage

### Rule 2.5 - Multiple immutable references
- **You CAN have multiple immutable references to the same data at once**
  - Doesn't affect anyone else, since it won't change

### Further notes
- Interesting comments from Rust By Example [here][by-example-aliasing]
  - Data can be immutably borrowed however many times you like
  - While immutably borrowed, can't be mutably borrowed
  - Data can have *only 1 mutable borrow at a time*

## More on References
- **A reference's scope starts from where it's introduced, and ends at the last time it's used**

[by-example-aliasing]: https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html#aliasing

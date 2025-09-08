# Common Programming Concepts
## Intro
- Concepts common acrosss languages
- How they appear in Rust
  - e.g. variables, basic types, functions, comments, control flow

## Keywords
- Set of words reserved for use by the language
- Can't be names of variables, functions (without using the [raw identifier syntax][raw-id-syntax])
- Some keywords don't do anything at present, but are reserved for future use

## Variables & Mutability
### Variables
- Immutable by default
  - Important for safety & easy concurrency
  - Good to signal intentions (and guarantees) to other devs
- Do not be dissuaded by compiler errors
- *Compile-time* errors are important here
  - If one part of code assumes value never changes
  - And another part changes it
  - Confusing and incorrect things happen
    - (especially when stuff only changes *sometimes*)

### Constants
- Declare with the `const` keyword (no `let`)
- Can't use `mut` with em
  - Always immutable
- Always require type annotations
- Can be defined in any scope (incl. global)
- Valid for entire program lifetime (`'static` lifetime)
- Should be UPPER_SNAKE_CASE
- Can only be set to a *constant expression* (compile-time computable expr.)
  - See [here for more][const-eval]
- Good for a lotta uses
  - Conveying intent/usage of something (no magic numbers pls)
  - Anti-hardcode

### Shadowing
- New variable, same name (as another in-scope)
- See [the code](./vars_and_mut/src/main.rs)
- Not the same as `mut`
  - Can sorta allow us to have a new immutable variable, after doing some operations on it

## Data Types
- All values have a *data type*
- Rust is *statically typed* (all data types known at compile time)

### Scalar
- Represents a single value

#### Integers
- Number without fractional component
- Sizes 8, 16, 32, 64, 128, `size` (architecture dependent)
- Rust's default is `u32`
- Use `isize`/`usize` when indexing collections

##### Literals
- Decimal: `982`
- Hex: `0xff`
- Octal: `0o77`
- Binary: `0b1100`
- Byte (`u8` only): `b'A'`
- Using digits separator: `1_000`

##### Signed
- `-2^(n-1) <= x <= 2^(n-1) - 1`

##### Unsigned
- `0 <= x <= 2^n - 1`

##### Overflow
- Debug builds panic on int overflows at runtime
- Release builds don't
  - Performs "2's complement wrapping instead"
  - Probably not great to rely on int overflow behaviour
- Handle overflows explicitly with
  - `wrapping_*` methods (e.g. `wrapping_add`)
  - Return `None` for overflows in `checked_*` methods
  - Return `Bool` for overflows with the `overflowing_*` methods
  - Saturate with the `saturating_*` methods

#### Floating-point numbers
- Decimal no's
- `f32` & `f64`
- Default is `f64`
  - Roughly the same speed as `f32` on modern CPUs apparently
- Represented by the *IEEE-754 standard*

##### Numeric Operations
- `+, -, *, /`
- `%` too for modulus/remainder
- Int division truncates towards 0 (to the nearest int)

#### Booleans
- `bool`s are 1 byte
- Great for conditionals

#### Characters
- `char` = primitive alphabetic type
- Specify char literals with single quotes (`'`)
- **Rust's char type is 4 bytes in size**
  - Unicode scalar value - not just ASCII
  - U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
- Intuition for chars mighn't match what you're thinkin here
  - Discussed more in chapter 8

### Compound
- Groups values into one type
- 2 primitives for this exist

#### Tuple
- General way to group together values with a variety of types
- Fixed length
  - Can't grow/shrink once declared
- See [the code][vars-code] for more info

#### Array
- See [the code][vars-code] for more info

##### Accessing Array Elements
##### Invalid Array Element Access

## Functions
## Comments
## Control Flow

<!-- Links -->
[raw-id-syntax]: https://doc.rust-lang.org/book/appendix-01-keywords.html
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
[vars-code]: ./vars_and_mut/src/main.rs

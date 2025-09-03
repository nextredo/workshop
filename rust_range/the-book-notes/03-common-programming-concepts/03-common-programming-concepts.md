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
## Functions
## Comments
## Control Flow

<!-- Links -->
[raw-id-syntax]: https://doc.rust-lang.org/book/appendix-01-keywords.html
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html

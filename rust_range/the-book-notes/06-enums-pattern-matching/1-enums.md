# Enums
- Spicy variant / type-safe union type baked into the language
- `Option` enum
  - A value that's something or nothing
- Encodes meaning with data
- Pattern matching
  - For running different code based on the enum
  - `match`
  - `if let`

## Overview
- Structs
  - Group related fields & data
    - e.g. Rectangle with fields and length
- Enums however, provide a way of saying a value is one of a set of possible values

## IP example
- Two standards for IP addresses (v4, v6)
- Therefore, IPs can only be 1 of 2 forms
- We can *enumerate* all possible values
  - Enum can only be 1 of its variants
  - But these are *variants* after all
  - So, code handling it should roughly act the same
    - Especially for things that apply to all IP addresses
- See [the source](./enumprog/src/main.rs) for more

## `Option`
- Very common scenario
  - A value could be something
  - Or it could be nothing
- An example is:
  - Getting the first item in a list
  - If the list is empty, you get nothing
  - If it has at least one item, you get something
- Baking this concept into the type system is good, as:
  - The compiler can check you've handled all cases
  - Avoiding runtime errors
  - Prevents extremely common bugs found in other languages

### Language design
- Is both a function of features you include
- And those you don't
- Here, we exclude `null` types
- Languages with null, variables can always either be `null` or not-`null`

> There is a great quote from Tony Hoare here<br>
> From his 2009 presentation "Null References: The Billion Dollar Mistake"<br>
> I won't repeat it here, but it's great. See the docs bro<br>

- Nulls cause heaps of errors
  - The property is pervasive
  - And not often checked that it's handled at compile/interpretation time
- Having type safe checking is better than sentinel/flag values like `null`

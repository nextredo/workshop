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

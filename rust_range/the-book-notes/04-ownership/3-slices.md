# Slice
- *Reference* to a contiguous sequence of elements in a *collection*
  - No ownership
- **String slice ranges must occur at valid UTF-8 character boundaries**
  - Program will exit with an error if not

## Other places slices may be seen
### String Literals
- `&str`

### Parameters
- Prefer `&str` over `&String` (deref coercions)

### Other slices
- Arrays

## Idiomatic Rust
- A function shouldn't take ownership unless it has to


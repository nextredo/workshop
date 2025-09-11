# This Chapter - Ownership
- Rust's most unique feature
- Deep implications
- How we achieve memory safety without
  - Garbage collection
  - Manual memory management

## Ownership
>P.S. I wonder if this is similar to ownership & borrowing in C++<br>
>e.g. `std::unique_ptr` and `std::move`

- Zero-cost abstraction
- Other languages use:
  - Garbage collection
  - Explicit allocation & freeing
- If ownership rules are violated, program won't compile

### The Stack & The Heap
#### Stack
- Systems programming type beat
- Stack is LIFO (last in, first out)
- Add = push
- Take = pop
- Fixed sizes required (sans `alloca()`)

#### Heap
- Allocate some amount wherever, get a pointer to it
- Pointer is a fixed size (lives on stack)

#### Comparisons
- Stack doesn't require an "allocation" process (faster)
- Heap data access requires a pointer dereference

#### Ownership's role
- Heap-focused, makes usage easier
- Minimise duplicate data
- Clean up unused data

---
### **Ownership's Rules**
1. Each value is rust has an *owner*
1. There can only be one owner at a time
1. When the owner goes out of scope, the value is dropped

### Scoping
- Range in which an item is valid
- Valid when *in scope*
- Goes *out of scope* and becomes invalid

```rust
{                 // s invalid (not declared yet)
    let s = "hi"; // s valid from here onwards

    // use s
}                 // s no longer valid (out of scope)
```

## Borrowing
## Slices
## Data layout

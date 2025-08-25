# Guessing Game
## Setting Up a New Project
- See [the last chapter](../01-getting-started/01-getting-started.md)

## Project
- See [the source file](./guessing_game/src/main.rs)
- See [the cargo toml file](./guessing_game/Cargo.toml)

### Dependencies
- When including new dependencies (`cargo build`)
- Cargo fetches from the *registry* (copy of <https://crates.io/>)
- Cargo downloads crates it needs to (following dependency tree)
- Cargo compiles the external crates
- Then compiles the project with those crates available
- Intelligent rebuilds - only makes what it has to again
    - e.g. if dependencies or src don't change, gg

#### `Cargo.lock`
- Ensures build reproducability
- Cargo only uses dependencies specified until indicated otherwise
- `Cargo.lock` created during first `cargo build` run
  - Cargo figures out all dependency versions, writes them to `Cargo.lock`
  - Ensures Cargo doesn't have to figure out versions again next build
- `Cargo.lock` often part of source version control system
  - Ensures builds are reproducable until explicitly changed, as a commit

#### `cargo update`
- Ignores `Cargo.lock`
- Figures out all latest versions that fit specs in `Cargo.toml`
- Writes new versions to `Cargo.lock`

#### 📦️ More Cargo
- Cargo discussed more in Chapter 14
- [Cargo docs][cargo-docs]

<!--- Links -->
[cargo-docs]: https://doc.rust-lang.org/cargo/

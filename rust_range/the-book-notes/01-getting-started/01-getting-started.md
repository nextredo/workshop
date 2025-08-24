# Getting Started
## 1.1 Installation
### Installation on Linux

```bash
# Install
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# Update
rustup update

# Check compiler version
rustc --version

# Uninstall with
rustup self uninstall

# Open docs locally
rustup doc
```

## 1.2 Hello, World!
- See [here](./main.rs)
- Rust is an *ahead-of-time* compiled language
  - Unlike interpreted languages (`python`, `ruby` etc.)

## 1.3 Hello, Cargo!
>[Cargo's documentation](cargo-docs)

### Intro
- Rust's build system & package manager
  - Handles dependencies

```bash
cargo --version

# Create a new project
  # Makes a new dir
  # Makes a new git repo
    # Not if it's already in a git repo
    # or ran with `--vcs=none`
cargo new 'project-name'
```

### `Cargo.toml`
- Format is [TOML][toml]
  - `[section]`
  - `key = "value"`

### Cargo
- [Cargo docs][cargo-docs]
- Expects source files in `src/`
- Top level directory is for:
  - READMEs, licensing, config files, etc.

### Building
```bash
# All these commands are OS-agnostic

# Build only
# Debug build by default
# Build an (optimised) release with --release
# Doesn't recompile if source files have changed
cargo build

# Build + run
cargo run

# Check it builds (don't make executable)
# Much faster that build
cargo check
```

#### `Cargo.lock`
- Tracks exact dependency versions

#### Convention
- Better than `rustc` for multi-file projects, dependencies

<!-- Links -->
[toml]: https://toml.io/
[cargo-docs]: https://doc.rust-lang.org/cargo/

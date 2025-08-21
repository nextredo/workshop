# Editions
## Language Development
- Language & compiler have a 6-week release cycle
- Editions have a 3-year release cycle
  - (mapped into the lang & compiler release cycle)
  - Existing editions
    - 2015, 2018, 2021, 2024

### What Editions Are
- Accumulated incremental changes
- Clearly packaged full tooling, docs, etc. update

## Editions & Cargo
- Uses `2015` if no `edition` key in `Cargo.toml`
  - Will alwaus use `edition` key otherwise
- Editions can contain incompatible changes
  - (e.g. new keyword, may break existing var names)
- Prev. editions always supported by newer compiler versions

### Dependencies
- Edition just changes how code's parsed
- Fine to use a Rust 2015 project with a Rust 2018 library
- Fine to use a Rust 2015 library with a Rust 2018 project

### Upgrades
- `cargo fix` to auto-upgrade code

## More Info
- [Edition Guide](edition-guide)


<!--- Links -->
[edition-guide]: https://doc.rust-lang.org/stable/edition-guide/

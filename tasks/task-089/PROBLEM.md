# task-089 — A full tray · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return true exactly when value tiles fill a whole number of trays of 5, including zero trays.

```rust
pub fn a_full_tray_5(issued: u16, returned: u16) -> bool
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `a_full_tray_5(4, 0)` gives `false`.
- `a_full_tray_5(0, 2)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-089/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-089`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

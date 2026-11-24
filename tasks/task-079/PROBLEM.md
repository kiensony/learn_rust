# task-079 — A full tray · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return true exactly when value tiles fill a whole number of trays of 3, including zero trays.

```rust
pub fn a_full_tray_3(first: u16, second: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `a_full_tray_3(4, 0)` gives `false`.
- `a_full_tray_3(0, 6)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-079/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-079`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

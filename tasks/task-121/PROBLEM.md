# task-121 — Fit a signed value into a byte

The caller needs an unsigned byte. Accept values from 0 through 255. Return Some(byte) on success and None outside the range. Rust’s try_from checks the conversion, and ok() keeps its successful value in an Option.

```rust
pub fn as_byte(reading: i16) -> Option<u8>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `as_byte(42)` gives `Some(42)`.
- `as_byte(255)` gives `Some(255)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-121/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-121`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

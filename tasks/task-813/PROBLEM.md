# task-813 — Classify a failed sensor reading

Trim the input and classify an empty field as Blank. Parse nonempty input as a u16, mapping parse failures to Number. A parsed value above 255 is Range. Return the remaining values as u8.

```rust
pub fn reading(text: &str) -> Result<u8, ReadingError>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `reading(" 7 ")` gives `Ok(7)`.
- `reading("256")` gives `Err(ReadingError::Range)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-813/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-813`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

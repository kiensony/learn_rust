# task-157 — Convert a signed duration without wrapping

Return Some(milliseconds) for a nonnegative number of seconds whose millisecond count fits u32. Negative inputs and counts too large for u32 both yield None. There are 1000 milliseconds per second.

```rust
pub fn milliseconds(seconds: i32) -> Option<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `milliseconds(2)` gives `Some(2000)`.
- `milliseconds(-1)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-157/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-157`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

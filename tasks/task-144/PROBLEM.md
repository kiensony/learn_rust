# task-144 — Round up · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return the smallest multiple of 8 at least as large as value.

```rust
pub fn round_up_8(start: u16, end: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `round_up_8(4, 0)` gives `8`.
- `round_up_8(13, 1)` gives `16`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-144/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-144`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

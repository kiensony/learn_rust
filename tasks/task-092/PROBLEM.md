# task-092 — Opening balance · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. The register starts with 8 tokens. Add the delivered tokens.

```rust
pub fn opening_balance_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `opening_balance_8(4, 0)` gives `12`.
- `opening_balance_8(13, 1)` gives `20`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-092/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-092`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

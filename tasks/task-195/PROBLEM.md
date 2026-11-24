# task-195 — Service interval · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Require a positive multiple of 8.

```rust
pub fn service_interval_8(start: u16, end: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `service_interval_8(4, 0)` gives `false`.
- `service_interval_8(8, 0)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-195/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-195`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

# task-191 — Last ticket · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Allow amounts strictly smaller than 8.

```rust
pub fn last_ticket_8(start: u16, end: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `last_ticket_8(4, 0)` gives `true`.
- `last_ticket_8(13, 1)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-191/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-191`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

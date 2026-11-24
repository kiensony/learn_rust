# task-201 — Last ticket · limited supply

A request is limited by available stock. Use the smaller count as value. Allow amounts strictly smaller than 10.

```rust
pub fn last_ticket_10(requested: u16, available: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `last_ticket_10(65535, 4)` gives `true`.
- `last_ticket_10(13, 14)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-201/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-201`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

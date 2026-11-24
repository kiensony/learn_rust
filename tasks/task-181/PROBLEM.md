# task-181 — Last ticket · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Allow amounts strictly smaller than 5.

```rust
pub fn last_ticket_5(issued: u16, returned: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `last_ticket_5(4, 0)` gives `true`.
- `last_ticket_5(13, 1)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-181/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-181`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

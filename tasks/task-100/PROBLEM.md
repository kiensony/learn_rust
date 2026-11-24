# task-100 — Ticket text · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Create a text label with the prefix "B-8/" followed by the decimal value.

```rust
pub fn ticket_text_8(start: u16, end: u16) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `ticket_text_8(4, 0)` gives `"B-8/4"`.
- `ticket_text_8(13, 1)` gives `"B-8/12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-100/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-100`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

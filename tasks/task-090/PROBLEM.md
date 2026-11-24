# task-090 — Ticket text · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Create a text label with the prefix "B-5/" followed by the decimal value.

```rust
pub fn ticket_text_5(issued: u16, returned: u16) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `ticket_text_5(4, 0)` gives `"B-5/4"`.
- `ticket_text_5(13, 1)` gives `"B-5/12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-090/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-090`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

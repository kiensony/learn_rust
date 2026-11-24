# task-110 — Ticket text · limited supply

A request is limited by available stock. Use the smaller count as value. Create a text label with the prefix "B-10/" followed by the decimal value.

```rust
pub fn ticket_text_10(requested: u16, available: u16) -> String
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `ticket_text_10(65535, 4)` gives `"B-10/4"`.
- `ticket_text_10(13, 14)` gives `"B-10/13"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-110/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-110`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

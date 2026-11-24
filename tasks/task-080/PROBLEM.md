# task-080 — Ticket text · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Create a text label with the prefix "B-3/" followed by the decimal value.

```rust
pub fn ticket_text_3(first: u16, second: u16) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `ticket_text_3(4, 0)` gives `"B-3/4"`.
- `ticket_text_3(13, 3)` gives `"B-3/16"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-080/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-080`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

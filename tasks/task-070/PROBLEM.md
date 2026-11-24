# task-070 — Ticket text · one reading

Create a text label with the prefix "B-2/" followed by the decimal value.

```rust
pub fn ticket_text_2(value: u16) -> String
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `ticket_text_2(4)` gives `"B-2/4"`.
- `ticket_text_2(13)` gives `"B-2/13"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-070/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-070`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

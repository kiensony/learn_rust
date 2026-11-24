# task-116 — Saturating debit · one reading

Remove 2 units from value. Stop at zero.

```rust
pub fn saturating_debit_2(value: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `saturating_debit_2(4)` gives `2`.
- `saturating_debit_2(13)` gives `11`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-116/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-116`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

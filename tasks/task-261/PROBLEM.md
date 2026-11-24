# task-261 — Give a helper one job

A pack contains two tickets. Write a small private helper named tickets_per_pack that returns 2 as a u32. Call it from double_tickets and return the total number of tickets. Widen packs before multiplying.

```rust
pub fn double_tickets(packs: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `double_tickets(2)` gives `4`.
- `double_tickets(5)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-261/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-261`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

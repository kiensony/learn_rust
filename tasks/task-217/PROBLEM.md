# task-217 — Cycle ticks · one reading

A wheel starts at position zero on a ring of 4 positions. Advance 2 places value times and return its final position.

```rust
pub fn cycle_ticks_2(value: u8) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `cycle_ticks_2(4)` gives `0`.
- `cycle_ticks_2(13)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-217/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-217`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

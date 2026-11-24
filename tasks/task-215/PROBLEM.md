# task-215 — Digit count · one reading

Count the base-2 digits of value. Zero takes one digit.

```rust
pub fn digit_count_2(value: u8) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `digit_count_2(4)` gives `3`.
- `digit_count_2(13)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-215/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-215`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

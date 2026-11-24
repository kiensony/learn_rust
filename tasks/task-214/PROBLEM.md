# task-214 — Digit sum · one reading

Add all base-2 digits of value. Zero has digit sum zero.

```rust
pub fn digit_sum_2(value: u8) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `digit_sum_2(4)` gives `1`.
- `digit_sum_2(13)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-214/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-214`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

# task-115 — Top up · one reading

Return how many units to add to reach a multiple of 2. A full group needs zero.

```rust
pub fn top_up_2(value: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `top_up_2(4)` gives `0`.
- `top_up_2(13)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-115/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-115`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

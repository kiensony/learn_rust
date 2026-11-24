# task-167 — Three shelves · one reading

Return 0 below 2, 1 from 2 through 4 inclusive, and 2 above 4.

```rust
pub fn three_shelves_2(value: u16) -> u8
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `three_shelves_2(4)` gives `1`.
- `three_shelves_2(13)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-167/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-167`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

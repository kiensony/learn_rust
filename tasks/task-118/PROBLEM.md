# task-118 — Digit place · one reading

Read the base-2 digit in the second position from the right.

```rust
pub fn digit_place_2(value: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `digit_place_2(4)` gives `0`.
- `digit_place_2(2)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-118/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-118`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

# task-068 — Two counters · one reading

One dial counts in groups of 2. Return both the complete groups and the remainder.

```rust
pub fn two_counters_2(value: u16) -> (u16, u16)
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `two_counters_2(4)` gives `(2, 0,)`.
- `two_counters_2(13)` gives `(6, 1,)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-068/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-068`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

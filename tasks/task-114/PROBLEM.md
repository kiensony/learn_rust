# task-114 — Round up · one reading

Return the smallest multiple of 2 at least as large as value.

```rust
pub fn round_up_2(value: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `round_up_2(4)` gives `4`.
- `round_up_2(13)` gives `14`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-114/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-114`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

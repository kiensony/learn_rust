# task-213 — Running deposits · one reading

Deposit 2 more coins each day than the day before, starting with 2 on day one. Return total for value days.

```rust
pub fn running_deposits_2(value: u8) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `running_deposits_2(4)` gives `20`.
- `running_deposits_2(13)` gives `182`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-213/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-213`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

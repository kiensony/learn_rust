# task-120 — Fractional fee · one reading

Charge 2 percent of value. Discard any fraction of a cent.

```rust
pub fn fractional_fee_2(value: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `fractional_fee_2(4)` gives `0`.
- `fractional_fee_2(65535)` gives `1310`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-120/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-120`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

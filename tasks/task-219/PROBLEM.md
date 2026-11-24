# task-219 — Number of divisors · one reading

Count positive numbers from 1 through 2 that divide value evenly. Every positive number divides zero.

```rust
pub fn number_of_divisors_2(value: u8) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `number_of_divisors_2(4)` gives `2`.
- `number_of_divisors_2(13)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-219/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-219`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

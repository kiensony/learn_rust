# task-211 — Sum a countdown

Use a while loop to sum every positive integer from start down to one. A while loop repeats while its condition is true. Decrease the current value once per iteration and store the sum as u32.

```rust
pub fn countdown_total(start: u8) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `countdown_total(3)` gives `6`.
- `countdown_total(4)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-211/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-211`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

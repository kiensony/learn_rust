# task-210 — Build a count with a loop

Build one string with exactly times exclamation marks. A for loop repeats its block once per value in a range. The range 0..times excludes the end. Use push to add a single character to the growing String.

```rust
pub fn knocks(times: u8) -> String
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `knocks(1)` gives `"!"`.
- `knocks(4)` gives `"!!!!"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-210/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-210`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

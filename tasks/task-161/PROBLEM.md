# task-161 — Choose the door message

A branch chooses between two paths. Return the text locked when locked is true and open when it is false. Rust uses if followed by a condition and a block in braces. An else block handles the other case.

```rust
pub fn door(locked: bool) -> &'static str
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `door(true)` gives `"locked"`.
- `door(false)` gives `"open"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-161/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-161`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

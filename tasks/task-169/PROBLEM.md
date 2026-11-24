# task-169 — Status lamp · one reading

Return "low" below 2, "full" above 4, and "ready" in between (including the boundaries).

```rust
pub fn status_lamp_2(value: u16) -> &'static str
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `status_lamp_2(4)` gives `"ready"`.
- `status_lamp_2(13)` gives `"full"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-169/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-169`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

# task-216 — Doubling steps · one reading

Start at 2. Count doublings until you reach or exceed value.

```rust
pub fn doubling_steps_2(value: u8) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `doubling_steps_2(4)` gives `1`.
- `doubling_steps_2(13)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-216/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-216`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

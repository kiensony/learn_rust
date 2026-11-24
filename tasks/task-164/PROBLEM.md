# task-164 — Alarm band · one reading

Sound an alarm below 2 or above 4.

```rust
pub fn alarm_band_2(value: u16) -> bool
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `alarm_band_2(4)` gives `false`.
- `alarm_band_2(13)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-164/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-164`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

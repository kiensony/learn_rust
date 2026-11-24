# task-067 — Reverse a reading · one reading

Subtract value from 65535, then divide the difference by 2 using integer division.

```rust
pub fn reverse_a_reading_2(value: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `reverse_a_reading_2(4)` gives `32765`.
- `reverse_a_reading_2(13)` gives `32761`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-067/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-067`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

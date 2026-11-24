# task-111 — Measure a signed temperature change

Find the change from before to after. Temperatures can be negative. An i8 ranges from -128 to 127, while their difference may need a wider type. Use i16::from on each reading before subtracting.

```rust
pub fn change(before: i8, after: i8) -> i16
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `change(2, 5)` gives `3`.
- `change(4, -2)` gives `-6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-111/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-111`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

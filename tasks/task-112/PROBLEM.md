# task-112 — Read quarter-degree measurements

A sensor measures in quarters of one degree. Convert the signed reading to floating point with f64::from and return the value in degrees. Every quarter degree is exactly representable by f64.

```rust
pub fn degrees(quarters: i16) -> f64
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `degrees(3)` gives `0.75`.
- `degrees(-5)` gives `-1.25`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-112/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-112`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

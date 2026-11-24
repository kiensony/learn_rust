# task-137 — Keep the fractional part of the average

Return the average of the two values as an f64. The current program loses half units on both positive and negative inputs. Use a test with an odd sum to isolate the first operation that loses information.

```rust
pub fn midpoint(first: i16, second: i16) -> f64
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `midpoint(2, 5)` gives `3.5`.
- `midpoint(0, 1)` gives `0.5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-137/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-137`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

# task-249 — Number of divisors · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Count positive numbers from 1 through 8 that divide value evenly. Every positive number divides zero.

```rust
pub fn number_of_divisors_8(start: u8, end: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `number_of_divisors_8(4, 0)` gives `3`.
- `number_of_divisors_8(13, 1)` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-249/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-249`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

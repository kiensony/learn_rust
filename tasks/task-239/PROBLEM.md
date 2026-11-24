# task-239 — Number of divisors · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Count positive numbers from 1 through 5 that divide value evenly. Every positive number divides zero.

```rust
pub fn number_of_divisors_5(issued: u8, returned: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `number_of_divisors_5(4, 0)` gives `3`.
- `number_of_divisors_5(13, 1)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-239/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-239`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

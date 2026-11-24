# task-229 — Number of divisors · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Count positive numbers from 1 through 3 that divide value evenly. Every positive number divides zero.

```rust
pub fn number_of_divisors_3(first: u8, second: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `number_of_divisors_3(4, 0)` gives `2`.
- `number_of_divisors_3(0, 6)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-229/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-229`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

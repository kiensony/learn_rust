# task-1792 — Token offset · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit unit` to obtain the units. Observe every input byte increased by the generated value (currently 5). Do not hard-code the generated value.

Return the sum of all observations, widening each one before adding. Use the supplied procedural macro at compile time. During review, point to the operation that enforces its contract.

```rust
pub fn token_offset_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `token_offset_total(&[])` gives `0`.
- `token_offset_total(&[4])` gives `9`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1792/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1792`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

# task-1791 — Token offset · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit unit` to obtain the units. Observe every input byte increased by the generated value (currently 5). Do not hard-code the generated value.

Return the observations in input order, including duplicates. Use the supplied procedural macro at compile time. During review, point to the operation that enforces its contract.

```rust
pub fn token_offset_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `token_offset_trace(&[])` gives `vec![]`.
- `token_offset_trace(&[4])` gives `vec![9u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1791/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1791`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

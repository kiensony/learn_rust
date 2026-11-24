# task-1783 — Token multiplier · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit` to obtain the units. Observe every input byte multiplied by the generated value (currently 4). Do not hard-code the generated value.

Return Some((minimum, maximum)) or None for an empty trace. Use the supplied procedural macro at compile time. During review, point to the operation that enforces its contract.

```rust
pub fn token_multiplier_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `token_multiplier_bounds(&[])` gives `None`.
- `token_multiplier_bounds(&[4])` gives `Some((16u32, 16u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1783/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1783`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

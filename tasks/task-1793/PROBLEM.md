# task-1793 — Token offset · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit unit` to obtain the units. Observe every input byte increased by the generated value (currently 5). Do not hard-code the generated value.

Return Some((minimum, maximum)) or None for an empty trace. Use the supplied procedural macro at compile time. During review, point to the operation that enforces its contract.

```rust
pub fn token_offset_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `token_offset_bounds(&[])` gives `None`.
- `token_offset_bounds(&[4])` gives `Some((9u32, 9u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1793/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1793`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

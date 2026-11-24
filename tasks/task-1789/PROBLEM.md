# task-1789 — Token multiplier · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit` to obtain the units. Observe every input byte multiplied by the generated value (currently 4). Do not hard-code the generated value.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the supplied procedural macro at compile time. During review, point to the operation that enforces its contract.

```rust
pub fn token_multiplier_longest_run(input: &[u8]) -> u64
```

## Public examples

- `token_multiplier_longest_run(&[])` gives `0`.
- `token_multiplier_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1789/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1789`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

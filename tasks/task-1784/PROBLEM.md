# task-1784 — Token multiplier · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

The provided arity::arity! procedural macro expands its input into the number of top-level token trees. Invoke it on `unit unit unit unit` to obtain the units. Observe every input byte multiplied by the generated value (currently 4). Do not hard-code the generated value.

Keep the first observation and every later one strictly above all earlier observations. Use the supplied procedural macro at compile time. During review, point to the operation that enforces its contract.

```rust
pub fn token_multiplier_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `token_multiplier_peaks(&[])` gives `vec![]`.
- `token_multiplier_peaks(&[4])` gives `vec![16u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1784/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1784`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

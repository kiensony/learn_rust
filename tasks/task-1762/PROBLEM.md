# task-1762 — Generated offset · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte increased by the generated value (currently 7). Do not hard-code the generated value.

Return the sum of all observations, widening each one before adding. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn generated_offset_total(input: &[u8]) -> u64
```

## Public examples

- `generated_offset_total(&[])` gives `0`.
- `generated_offset_total(&[4])` gives `11`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1762/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1762`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

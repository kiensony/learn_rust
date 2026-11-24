# task-1759 — Build multiplier · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte multiplied by the generated value (currently 3). Do not hard-code the generated value.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_multiplier_longest_run(input: &[u8]) -> u64
```

## Public examples

- `build_multiplier_longest_run(&[])` gives `0`.
- `build_multiplier_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1759/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1759`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

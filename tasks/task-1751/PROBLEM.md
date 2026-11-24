# task-1751 — Build multiplier · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte multiplied by the generated value (currently 3). Do not hard-code the generated value.

Return the observations in input order, including duplicates. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_multiplier_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `build_multiplier_trace(&[])` gives `vec![]`.
- `build_multiplier_trace(&[4])` gives `vec![12u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1751/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1751`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

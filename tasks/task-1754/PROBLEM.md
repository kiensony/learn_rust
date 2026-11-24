# task-1754 — Build multiplier · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte multiplied by the generated value (currently 3). Do not hard-code the generated value.

Keep the first observation and every later one strictly above all earlier observations. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_multiplier_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `build_multiplier_peaks(&[])` gives `vec![]`.
- `build_multiplier_peaks(&[4])` gives `vec![12u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1754/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1754`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

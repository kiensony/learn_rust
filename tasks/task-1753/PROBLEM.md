# task-1753 — Build multiplier · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte multiplied by the generated value (currently 3). Do not hard-code the generated value.

Return Some((minimum, maximum)) or None for an empty trace. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_multiplier_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `build_multiplier_bounds(&[])` gives `None`.
- `build_multiplier_bounds(&[4])` gives `Some((12u32, 12u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1753/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1753`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

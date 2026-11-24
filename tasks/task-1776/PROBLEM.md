# task-1776 — Build mask · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte bitwise AND with the generated value (currently 63). Do not hard-code the generated value.

Count adjacent pairs for which the second observation is strictly smaller. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_mask_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `build_mask_descending_steps(&[])` gives `0`.
- `build_mask_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1776/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1776`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

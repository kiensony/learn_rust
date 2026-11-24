# task-1772 — Build mask · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte bitwise AND with the generated value (currently 63). Do not hard-code the generated value.

Return the sum of all observations, widening each one before adding. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_mask_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `build_mask_total(&[])` gives `0`.
- `build_mask_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1772/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1772`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

# task-1778 — Build mask · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte bitwise AND with the generated value (currently 63). Do not hard-code the generated value.

Return the absolute differences between each adjacent pair, in order. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_mask_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `build_mask_steps(&[])` gives `vec![]`.
- `build_mask_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1778/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1778`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

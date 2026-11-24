# task-1774 — Build mask · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte bitwise AND with the generated value (currently 63). Do not hard-code the generated value.

Keep the first observation and every later one strictly above all earlier observations. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_mask_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `build_mask_peaks(&[])` gives `vec![]`.
- `build_mask_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1774/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1774`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

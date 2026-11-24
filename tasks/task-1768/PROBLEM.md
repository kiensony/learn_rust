# task-1768 — Generated offset · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte increased by the generated value (currently 7). Do not hard-code the generated value.

Return the absolute differences between each adjacent pair, in order. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn generated_offset_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `generated_offset_steps(&[])` gives `vec![]`.
- `generated_offset_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1768/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1768`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

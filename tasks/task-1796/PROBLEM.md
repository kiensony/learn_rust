# task-1796 — Build multiplier · threshold crossing

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte multiplied by the generated value (currently 3). Do not hard-code the generated value.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use a Cargo build script and env!. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn build_multiplier_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `build_multiplier_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 9))`.
- `build_multiplier_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1796/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1796`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

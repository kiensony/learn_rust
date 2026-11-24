# task-1779 — Build mask · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte bitwise AND with the generated value (currently 63). Do not hard-code the generated value.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_mask_longest_run(input: &[u8]) -> u64
```

## Public examples

- `build_mask_longest_run(&[])` gives `0`.
- `build_mask_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1779/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1779`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

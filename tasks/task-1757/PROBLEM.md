# task-1757 — Build multiplier · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte multiplied by the generated value (currently 3). Do not hard-code the generated value.

Return distinct observations in ascending order. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn build_multiplier_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `build_multiplier_distinct(&[])` gives `vec![]`.
- `build_multiplier_distinct(&[4])` gives `vec![12u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1757/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1757`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

# task-1770 — Generated offset · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read the TRACK_UNITS value exported by build.rs. It comes from units.txt. Observe every input byte increased by the generated value (currently 7). Do not hard-code the generated value.

Return the observations in reverse order. Retain duplicate observations. Use a Cargo build script and env!. During review, point to the operation that enforces its contract.

```rust
pub fn generated_offset_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `generated_offset_reverse(&[])` gives `vec![]`.
- `generated_offset_reverse(&[4])` gives `vec![11u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1770/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1770`.

[Module reading](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

# task-1350 — Packed booleans · replay mismatch

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the number of one bits in each parsed byte.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use str::parse and explicit handling of parse errors. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn packed_booleans_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `packed_booleans_replay_mismatch("4,9,2,2,0", &[1, 2, 1, 1])` gives `Some(4)`.
- `packed_booleans_replay_mismatch("", &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1350/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1350`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

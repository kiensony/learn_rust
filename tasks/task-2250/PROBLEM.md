# task-2250 — Borrowed windows · replay mismatch

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe the big-endian u16 represented by each overlapping two-byte window, including windows within a multibyte character. Allocate only storage needed for the observations or result.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use borrowed slices or indices into the original input. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn borrowed_windows_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `borrowed_windows_replay_mismatch("a b c\n\r\nup\tdown", &[24864, 8290, 25120, 8291, 25354, 2573, 3338, 2677, 30064, 28681, 2404, 25711, 28535])` gives `Some(13)`.
- `borrowed_windows_replay_mismatch("", &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2250/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2250`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

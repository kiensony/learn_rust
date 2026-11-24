# task-2249 — Zero copy lines · frequent readings

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe lengths in bytes of logical lines, stripping LF or CRLF. Allocate only storage needed for the observations or result.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use borrowed slices or indices into the original input. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn zero_copy_lines_frequent_readings(input: &str, keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `zero_copy_lines_frequent_readings("nine\nq\nthree\nx\n", 3)` gives `vec![1u32, 4u32, 5u32]`.
- `zero_copy_lines_frequent_readings("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2249/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2249`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

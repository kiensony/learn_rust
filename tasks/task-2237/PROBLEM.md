# task-2237 — Zero copy lines · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe lengths in bytes of logical lines, stripping LF or CRLF. Allocate only storage needed for the observations or result.

Return distinct observations in ascending order. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn zero_copy_lines_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `zero_copy_lines_distinct("")` gives `vec![]`.
- `zero_copy_lines_distinct("a")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2237/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2237`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

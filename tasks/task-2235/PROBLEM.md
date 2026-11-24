# task-2235 — Zero copy lines · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe lengths in bytes of logical lines, stripping LF or CRLF. Allocate only storage needed for the observations or result.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn zero_copy_lines_changes(input: &str) -> Vec<u32>
```

## Public examples

- `zero_copy_lines_changes("")` gives `vec![]`.
- `zero_copy_lines_changes("a")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2235/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2235`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

# task-2215 — Borrowed offsets · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe zero-based byte offsets of LF terminators. Allocate only storage needed for the observations or result.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_offsets_changes(input: &str) -> Vec<u32>
```

## Public examples

- `borrowed_offsets_changes("")` gives `vec![]`.
- `borrowed_offsets_changes("a")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2215/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2215`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

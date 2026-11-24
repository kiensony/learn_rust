# task-2225 — Borrowed scalars · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe UTF-8 byte offsets of Unicode scalar boundaries, excluding the final boundary. Allocate only storage needed for the observations or result.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_scalars_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `borrowed_scalars_changes("")` gives `vec![]`.
- `borrowed_scalars_changes("a")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2225/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2225`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

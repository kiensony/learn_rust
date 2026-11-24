# task-484 — Select a safe viewing window

Borrow the width bytes beginning at start. An empty window at the end is valid. Return None for an out-of-bounds or overflowing range.

```rust
pub fn window(bytes: &[u8], start: usize, width: usize) -> Option<&[u8]>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `window(&[1,2,3,4], 1, 2)` gives `Some(&[2,3][..])`.
- `window(&[7,8,9], 2, 1)` gives `Some(&[9][..])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-484/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-484`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)

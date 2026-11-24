# task-230 — Count chimes · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Count positive multiples of 5 up to and including value.

```rust
pub fn count_chimes_5(issued: u8, returned: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `count_chimes_5(4, 0)` gives `0`.
- `count_chimes_5(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-230/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-230`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

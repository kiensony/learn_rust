# task-236 — Skip a closed platform

Count all open platforms in the entire slice. The starter stops checking after the first closed platform. Keep its loop and repair the control-flow instruction.

```rust
pub fn open_count(places: &[bool]) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `open_count(&[true, false, true])` gives `2`.
- `open_count(&[false, true])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-236/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-236`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

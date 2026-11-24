# task-632 — Use the dimensions of each panel

Find the total length of the four sides of a rectangular panel. Width and height vary independently. Widen both fields before the arithmetic.

```rust
pub fn border(panel: Panel) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `border(Panel { width: 3, height: 5 })` gives `16`.
- `border(Panel { width: 0, height: 0 })` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-632/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-632`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

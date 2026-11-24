# task-681 — Distinguish held buttons from new presses

A button sampled up always becomes Up. A down sample after Up becomes Pressed. Any other down sample becomes Held. Preserve all three observable states.

```rust
pub fn step(state: Button, down: bool) -> Button
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `step(Button::Up, true)` gives `Button::Pressed`.
- `step(Button::Pressed, true)` gives `Button::Held`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-681/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-681`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)

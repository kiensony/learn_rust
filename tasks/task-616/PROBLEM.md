# task-616 — Advance a meter through a method

Create a Meter starting at start. Advance it once per step using its supplied method and return the final reading. The meter saturates at 255.

```rust
pub fn after(start: u8, steps: &[u8]) -> u8
```

Implement the public function in `src/lib.rs`.

## Public examples

- `after(3, &[2, 1])` gives `6`.
- `after(20, &[5])` gives `25`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-616/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-616`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

# task-226 — Doubling steps · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Start at 3. Count doublings until you reach or exceed value.

```rust
pub fn doubling_steps_3(first: u8, second: u8) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `doubling_steps_3(4, 0)` gives `1`.
- `doubling_steps_3(13, 3)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-226/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-226`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

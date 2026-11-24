# task-176 — Override code · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Allow 3, 5, and no other code.

```rust
pub fn override_code_3(first: u16, second: u16) -> bool
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `override_code_3(4, 0)` gives `false`.
- `override_code_3(4, 1)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-176/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-176`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

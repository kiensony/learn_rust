# task-122 — Price in cents · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. At 3 cents per unit, compute the total price as a u32.

```rust
pub fn price_in_cents_3(first: u16, second: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `price_in_cents_3(4, 0)` gives `12`.
- `price_in_cents_3(13, 3)` gives `48`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-122/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-122`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

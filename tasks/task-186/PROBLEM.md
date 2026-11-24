# task-186 — Include the closing boundary

Fare is zero through age 4, two units from ages 5 through 17 inclusive and five units from age 18. Identify the first age where the starter charges the wrong price.

```rust
pub fn fare(age: u8) -> u8
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `fare(4)` gives `0`.
- `fare(17)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-186/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-186`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

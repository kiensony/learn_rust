# task-072 — Opening balance · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. The register starts with 3 tokens. Add the delivered tokens.

```rust
pub fn opening_balance_3(first: u16, second: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `opening_balance_3(4, 0)` gives `7`.
- `opening_balance_3(13, 3)` gives `19`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-072/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-072`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

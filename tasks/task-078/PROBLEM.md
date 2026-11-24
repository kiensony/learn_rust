# task-078 — Two counters · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. One dial counts in groups of 3. Return both the complete groups and the remainder.

```rust
pub fn two_counters_3(first: u16, second: u16) -> (u16, u16)
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `two_counters_3(4, 0)` gives `(1, 1,)`.
- `two_counters_3(13, 3)` gives `(5, 1,)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-078/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-078`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

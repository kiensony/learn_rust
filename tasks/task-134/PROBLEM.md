# task-134 — Round up · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return the smallest multiple of 5 at least as large as value.

```rust
pub fn round_up_5(issued: u16, returned: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `round_up_5(4, 0)` gives `5`.
- `round_up_5(13, 1)` gives `15`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-134/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-134`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

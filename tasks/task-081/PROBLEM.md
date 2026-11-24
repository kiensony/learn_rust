# task-081 — Count packs · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Each pack contains 5 pencils. Return the number of pencils in value packs.

```rust
pub fn count_packs_5(issued: u16, returned: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `count_packs_5(4, 0)` gives `20`.
- `count_packs_5(13, 1)` gives `60`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-081/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-081`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

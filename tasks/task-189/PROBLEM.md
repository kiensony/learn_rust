# task-189 — Status lamp · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return "low" below 5, "full" above 7, and "ready" in between (including the boundaries).

```rust
pub fn status_lamp_5(issued: u16, returned: u16) -> &'static str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `status_lamp_5(4, 0)` gives `"low"`.
- `status_lamp_5(13, 1)` gives `"full"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-189/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-189`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

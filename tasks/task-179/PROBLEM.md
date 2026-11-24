# task-179 — Status lamp · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return "low" below 3, "full" above 5, and "ready" in between (including the boundaries).

```rust
pub fn status_lamp_3(first: u16, second: u16) -> &'static str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `status_lamp_3(4, 0)` gives `"ready"`.
- `status_lamp_3(13, 3)` gives `"full"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-179/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-179`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

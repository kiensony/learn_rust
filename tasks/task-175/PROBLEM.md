# task-175 — Service interval · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Require a positive multiple of 3.

```rust
pub fn service_interval_3(first: u16, second: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `service_interval_3(4, 0)` gives `false`.
- `service_interval_3(0, 6)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-175/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-175`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

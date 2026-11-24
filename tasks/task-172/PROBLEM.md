# task-172 — Comfort zone · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Accept values from 3 through 5 inclusive.

```rust
pub fn comfort_zone_3(first: u16, second: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `comfort_zone_3(4, 0)` gives `true`.
- `comfort_zone_3(13, 3)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-172/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-172`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

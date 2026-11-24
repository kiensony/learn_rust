# task-223 — Running deposits · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Deposit 3 more coins each day than the day before, starting with 3 on day one. Return total for value days.

```rust
pub fn running_deposits_3(first: u8, second: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `running_deposits_3(4, 0)` gives `30`.
- `running_deposits_3(13, 3)` gives `408`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-223/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-223`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

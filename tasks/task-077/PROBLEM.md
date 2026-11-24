# task-077 — Reverse a reading · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Subtract value from 65535, then divide the difference by 3 using integer division.

```rust
pub fn reverse_a_reading_3(first: u16, second: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `reverse_a_reading_3(4, 0)` gives `21843`.
- `reverse_a_reading_3(13, 3)` gives `21839`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-077/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-077`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)

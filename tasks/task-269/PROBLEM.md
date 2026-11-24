# task-269 — Write your first unit test

Increase the value by one, stopping at 255. Add a unit test in src/lib.rs under #[cfg(test)] mod tests. Import the function with use super::bump. Put #[test] above a function named ceiling_stays_put. In its body compare bump(255) with 255 using assert_eq!. A reviewer will ask you to demonstrate this test with cargo test --lib ceiling_stays_put. An ordinary input should increase.

```rust
pub fn bump(value: u8) -> u8
```

Implement the public function in `src/lib.rs`.

## Public examples

- `bump(3)` gives `4`.
- `bump(9)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-269/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-269`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

# task-1012 — Swap generic endpoints

Reverse the two values in the owned tuple. Accept any T without requiring Clone, Copy, Default, or arithmetic capabilities.

```rust
pub fn swap_pair<T>(pair: (T,T)) -> (T,T)
```

Implement the public function in `src/lib.rs`.

## Public examples

- `swap_pair((1,4))` gives `(4,1)`.
- `swap_pair(("north","west"))` gives `("west","north")`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1012/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1012`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)

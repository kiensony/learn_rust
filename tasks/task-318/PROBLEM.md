# task-318 — Unpack a shipment

A caller hands over the label and quantity as one package. Move the label into the result and put the quantity first. Reuse the original String allocation.

```rust
pub fn unpack(package: (String, u16)) -> (u16, String)
```

Implement the public function in `src/lib.rs`.

## Public examples

- `unpack((String::from("tea"), 3))` gives `(3, String::from("tea"))`.
- `unpack((String::from("ink"), 21))` gives `(21, String::from("ink"))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-318/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-318`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

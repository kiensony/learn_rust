# task-368 — Inspect a crate label

Borrow the existing label. Return its UTF-8 byte count and whether all characters are ASCII. An empty string contains only ASCII bytes.

```rust
pub fn label_info(label: &String) -> (usize, bool)
```

Implement the public function in `src/lib.rs`.

## Public examples

- `label_info(&String::from("Lake"))` gives `(4, true)`.
- `label_info(&String::from("trail map"))` gives `(9, true)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-368/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-368`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

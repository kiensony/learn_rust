# task-1061 — Read an associated key

Return the primary key reference through the Keyed trait. The implementation chooses the key type. Keep the reference borrowed from the supplied source.

```rust
pub fn key_of<S: Keyed>(source: &S) -> &S::Key
```

Implement the public function in `src/lib.rs`.

## Public examples

- `key_of(&Record(4u8,9u8))` gives `&4`.
- `key_of(&Record("A","B"))` gives `&"A"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1061/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1061`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)

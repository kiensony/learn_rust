# task-1161 — Split a borrowed configuration entry

Return two slices borrowed from opposite sides of the first equals sign. Keep additional equals signs in the value. Return None when there is no separator.

```rust
pub fn setting(line: &str) -> Option<(&str,&str)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `setting("host=local")` gives `Some(("host","local"))`.
- `setting("a=b=c")` gives `Some(("a","b=c"))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1161/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1161`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)

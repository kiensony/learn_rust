# task-1127 — Format a foreign collection through a local type

Reproduce the orphan-rule error. Introduce a local newtype for the owned byte vector, implement Display on it, and return its formatted label. Always use the spelling “bytes”, including for one byte.

```rust
pub fn byte_label(bytes: Vec<u8>) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `byte_label(vec![2, 3])` gives `"2 bytes"`.
- `byte_label(vec![8])` gives `"1 bytes"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1127/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1127`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)

# task-1111 — Dispatch a heterogeneous signal list

Sample each source through its trait object. Preserve order and one output per input, including repeated sources and mixed implementation types.

```rust
pub fn sample_all(inputs: &[&dyn Source]) -> Vec<i16>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sample_all(&[&Constant(2), &Negate(3)])` gives `vec![2,-3]`.
- `sample_all(&[&Negate(-2)])` gives `vec![2]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1111/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1111`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)

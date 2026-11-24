# task-1317 — Nibble fields · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Return distinct observations in ascending order. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_fields_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `nibble_fields_distinct("")` gives `vec![]`.
- `nibble_fields_distinct("1")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1317/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1317`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

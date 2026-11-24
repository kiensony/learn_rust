# task-1344 — Packed booleans · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the number of one bits in each parsed byte.

Keep the first observation and every later one strictly above all earlier observations. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn packed_booleans_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `packed_booleans_peaks("")` gives `vec![]`.
- `packed_booleans_peaks("1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1344/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1344`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

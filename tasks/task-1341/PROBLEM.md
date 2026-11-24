# task-1341 — Packed booleans · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the number of one bits in each parsed byte.

Return the observations in input order, including duplicates. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn packed_booleans_trace(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packed_booleans_trace("")` gives `vec![]`.
- `packed_booleans_trace("1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1341/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1341`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

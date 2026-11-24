# task-1314 — Nibble fields · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Keep the first observation and every later one strictly above all earlier observations. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_fields_peaks(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `nibble_fields_peaks("")` gives `vec![]`.
- `nibble_fields_peaks("1")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1314/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1314`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

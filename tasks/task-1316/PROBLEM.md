# task-1316 — Nibble fields · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Count adjacent pairs for which the second observation is strictly smaller. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_fields_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `nibble_fields_descending_steps("")` gives `0`.
- `nibble_fields_descending_steps("1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1316/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1316`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

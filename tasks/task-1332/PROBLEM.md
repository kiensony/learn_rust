# task-1332 — Field indexes · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the zero-based indexes of parsed zero values, counting only valid fields.

Return the sum of all observations, widening each one before adding. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn field_indexes_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `field_indexes_total("")` gives `0`.
- `field_indexes_total("1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1332/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1332`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

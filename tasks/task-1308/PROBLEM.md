# task-1308 — Decimal fields · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe each parsed byte.

Return the absolute differences between each adjacent pair, in order. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn decimal_fields_steps(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `decimal_fields_steps("")` gives `vec![]`.
- `decimal_fields_steps("1")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1308/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1308`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

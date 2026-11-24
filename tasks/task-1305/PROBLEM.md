# task-1305 — Decimal fields · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe each parsed byte.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn decimal_fields_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `decimal_fields_changes("")` gives `vec![]`.
- `decimal_fields_changes("1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1305/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1305`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)

# task-1477 — Retry fixture · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 5xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return distinct observations in ascending order. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn retry_fixture_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `retry_fixture_distinct("")` gives `vec![]`.
- `retry_fixture_distinct("200")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1477/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1477`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

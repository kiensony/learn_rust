# task-1487 — Response boundaries · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe indexes of successful or redirect responses (codes below 400) in the accepted sequence. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return distinct observations in ascending order. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn response_boundaries_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `response_boundaries_distinct("")` gives `vec![]`.
- `response_boundaries_distinct("200")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1487/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1487`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

# task-1499 — Response boundaries · frequent readings

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe indexes of successful or redirect responses (codes below 400) in the accepted sequence. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use an integration test outside src and the crate's public API. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn response_boundaries_frequent_readings(input: &str, keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `response_boundaries_frequent_readings(" 302 204 503 400 201 ", 3)` gives `vec![0u32, 1u32, 4u32]`.
- `response_boundaries_frequent_readings("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1499/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1499`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

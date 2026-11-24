# task-1488 — Response boundaries · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe indexes of successful or redirect responses (codes below 400) in the accepted sequence. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the absolute differences between each adjacent pair, in order. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn response_boundaries_steps(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `response_boundaries_steps("")` gives `vec![]`.
- `response_boundaries_steps("200")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1488/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1488`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

# task-1489 — Response boundaries · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe indexes of successful or redirect responses (codes below 400) in the accepted sequence. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn response_boundaries_longest_run(input: &str) -> u64
```

## Public examples

- `response_boundaries_longest_run("")` gives `0`.
- `response_boundaries_longest_run("200")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1489/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1489`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

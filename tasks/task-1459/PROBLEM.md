# task-1459 — HTTP status fixture · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe the hundred-class digit of each accepted code. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn http_status_fixture_longest_run(input: &str) -> u64
```

## Public examples

- `http_status_fixture_longest_run("")` gives `0`.
- `http_status_fixture_longest_run("200")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1459/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1459`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

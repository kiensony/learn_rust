# task-034 — Publish an example target

Cargo discovers examples in the `examples` directory. Run `cargo run --example preview` inside the task, then edit that example so it prints `Preview only` on one line. The regular binary must continue to print `Full edition`.

From the repository directory, open `tasks/task-034/` and edit `examples/preview.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-034/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-034/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Full edition\n"`.
- Running `cargo run --example preview` produces standard output `"Preview only\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-034/Cargo.toml
```
